/// ⏱️ Imperative scene timeline built by `play` / `wait` calls.
#[derive(Clone, Debug, Default)]
pub struct SceneTimeline {
    segments: Vec<TimelineSegment>,
    cursor: f64,
}

#[derive(Clone, Debug)]
pub(crate) enum TimelineSegment {
    Play { start: f64, duration: f64 },
    Wait { start: f64, duration: f64 },
}

impl SceneTimeline {
    /// ▶️ Schedules an animation segment.
    pub fn play(&mut self, duration: f64) {
        let duration = duration.max(0.0);
        self.segments.push(TimelineSegment::Play { start: self.cursor, duration });
        self.cursor += duration;
    }

    /// ⏸️ Schedules a hold segment.
    pub fn wait(&mut self, duration: f64) {
        let duration = duration.max(0.0);
        self.segments.push(TimelineSegment::Wait { start: self.cursor, duration });
        self.cursor += duration;
    }

    /// ⏳ Total scene duration in seconds.
    pub fn total_duration(&self) -> f64 {
        self.cursor
    }

    /// 🎞️ Frame count at the configured frame rate.
    pub fn frame_count(&self, frame_rate: f64) -> u32 {
        let fps = frame_rate.max(1.0);
        (self.total_duration() * fps).ceil().max(1.0) as u32
    }

    /// 🕒 Scene time for a frame index.
    pub fn time_at_frame(&self, frame: u32, frame_rate: f64) -> f64 {
        let fps = frame_rate.max(1.0);
        let total = self.total_duration();
        let t = frame as f64 / fps;
        t.min(total)
    }

    /// 📋 Timeline segments for hashing.
    pub fn segments(&self) -> &[TimelineSegment] {
        &self.segments
    }
}
