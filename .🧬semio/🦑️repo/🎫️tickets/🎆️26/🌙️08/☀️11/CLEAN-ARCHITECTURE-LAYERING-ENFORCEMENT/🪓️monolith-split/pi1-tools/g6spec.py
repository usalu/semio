#!/usr/bin/env python3
"""🧱️ Spec of group G6: reactor-app and reactor-tasks (the lowest WP4 crates). reactor-tasks owns seven persistent-local statics (D8), moved out of the reactor root block as `pub static` (macro `$vis`)."""
from lib import *
import groupdrv as G

import statics as ST

TASKS = PLUG + "/⚛️reactor/⏲️tasks"
OWNED = ["REGISTRY", "REACTOR_EXECUTOR", "TASK_EXECUTOR", "INSTANCE_METADATA", "TASK_RECORDS", "TASK_RESUMES", "JOB_RENDER_BINDINGS"]
REGISTRY_TYPES = ["TaskRecordRegistry", "FixedResumeQueue", "InstanceMetadataRegistry", "JobRenderBindingRegistry"]
USES = """use std::cell::RefCell;
use semio_framework_plugin_actor_host::{executor, requests};
use tasks_instance_metadata::InstanceMetadataRegistry;
use tasks_render_bindings::JobRenderBindingRegistry;
use tasks_task_lane::{FixedResumeQueue, TaskRecordRegistry};
"""
widen_registry = ST.widen_types(REGISTRY_TYPES)
import_owned = ST.import_owned


class S(G.Spec):
    name = "g6"
    group = ["reactor-app", "reactor-tasks"]
    roots_b = {"reactor-app": PLUG + "/⚛️reactor/🔌️app/🦀️.rs"}
    hub_file = PLUG + "/🪝️mounts/⚛️wp4-reactor/🦀️.rs"
    hub_units = {}
    root_extra = {"reactor-app": "\npub use app_runtime_helpers::*;\n", "reactor-tasks": ST.owned_block(OWNED, USES)}
    extra_edits = {
        ST.ROOT: ST.leave_root({"semio_framework_plugin_reactor_tasks": OWNED}),
        TASKS + "/🧵️task-lane/🦀️.rs": lambda t: widen_registry(import_owned(OWNED)(t)),
        TASKS + "/🔗️extension-continuation/🦀️.rs": import_owned(OWNED),
        TASKS + "/🧷️render-bindings/🦀️.rs": widen_registry,
        TASKS + "/📇️instance-metadata/🦀️.rs": widen_registry,
    }


if __name__ == "__main__":
    G.main(S)
