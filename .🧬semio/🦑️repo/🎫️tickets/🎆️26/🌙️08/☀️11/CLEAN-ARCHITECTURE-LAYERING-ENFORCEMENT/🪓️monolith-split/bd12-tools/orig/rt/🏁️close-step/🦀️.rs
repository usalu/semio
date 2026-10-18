//! 🏁️ One bounded reactor close opportunity driven by the turn.

use semio_framework_plugin_reactor_core::ARMED_TIMERS;
use std::cell::Cell;
use semio_framework_plugin_reactor_tasks::{INSTANCE_METADATA, REGISTRY, TASK_RESUMES};
use semio_framework_plugin_reactor_tasks::tasks_fixed_slots::PLUGIN_REACTOR_INSTANCE_SLOTS;
use semio_framework_plugin_reactor_lifecycle::{REACTOR_CLOSES, REACTOR_CLOSE_CURSOR};
use semio_framework_plugin_reactor_tasks::tasks_task_lane::cancel_instance_tasks_step;
use semio_framework_plugin_actor_host::requests;
use crate as turn;

#[doc(hidden)]
pub fn step_reactor_close(cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
    REACTOR_CLOSES.with(|closes| {
        let Ok(mut closes) = closes.try_borrow_mut() else { return Ok(false) };
        let start = REACTOR_CLOSE_CURSOR.with(Cell::get);
        let Some(index) = (0..PLUGIN_REACTOR_INSTANCE_SLOTS).map(|offset| (start + offset) % PLUGIN_REACTOR_INSTANCE_SLOTS).find(|index| closes.slots.get(*index).is_some_and(|state| state.active && !state.complete)) else { return Ok(false) };
        REACTOR_CLOSE_CURSOR.with(|cursor| cursor.set((index + 1) % PLUGIN_REACTOR_INSTANCE_SLOTS));
        let state = closes.slots.get_mut(index).expect("selected exact active close");
        if !turn::original_receiving_sources_empty(state.key){return Ok(true)}
        let complete = if !state.command_ingress_complete {
            state.command_ingress_complete = turn::close_command_ingress_step(state.key,cx)?;
            false
        } else if !state.requests_complete {
            state.requests_complete = REGISTRY.with(|registry| registry.cancel_instance_step(&mut state.request_cursor) == requests::RequestCloseStep::Complete);
            false
        } else if !state.resumes_complete {
            state.resumes_complete = TASK_RESUMES.with(|resumes| resumes.borrow_mut().cancel_instance_step(state.instance, &mut state.resume_remaining));
            false
        } else if !state.tasks_complete {
            state.tasks_complete = cancel_instance_tasks_step(state.instance, &mut state.task_cursor);
            false
        } else if !state.timers_complete {
            state.timers_complete = ARMED_TIMERS.with(|timers| timers.borrow_mut().cancel_instance_step(state.instance, &mut state.timer_cursor));
            false
        } else if !state.metadata_complete {
            INSTANCE_METADATA.with(|metadata| {
                drop(metadata.borrow_mut().remove(state.instance));
            });
            state.metadata_complete = true;
            false
        } else {
            true
        };
        state.complete = complete;
        Ok(true)
    })
}
