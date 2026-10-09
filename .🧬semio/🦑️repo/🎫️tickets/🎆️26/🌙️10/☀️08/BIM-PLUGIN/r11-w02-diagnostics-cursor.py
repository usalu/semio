import sys
p = sys.argv[1]
s = open(p, encoding='utf-8', newline='').read()
def rep(old, new):
    global s
    assert s.count(old) == 1, (old, s.count(old))
    s = s.replace(old, new)
rep('''        let Some(at) = typed::resolve(&entry, self.tool.anchor(), self.last) else { return self.advance(ctx, &ToolEvent::Finish) };
        let pointer = Pointer { at, modifiers: Modifiers::default(), tolerance: typed::REACH };
        let mut step = Step::default();
        for event in [ToolEvent::Down(pointer), ToolEvent::Up(pointer), ToolEvent::Move(pointer)] {
            step = step.then(self.advance(ctx, &event).0);
            if step.refused.is_some() {
                break;
            }
        }
        (step, self.tool.preview(ctx))
    }
}''', '''        let Some(at) = typed::resolve(&entry, self.tool.anchor(), self.last) else { return self.advance(ctx, &ToolEvent::Finish) };
        self.click(ctx, at)
    }

    fn click(&mut self, ctx: &mut ToolContext<'_>, at: P) -> (Step, Preview) {
        let pointer = Pointer { at, modifiers: Modifiers::default(), tolerance: typed::REACH };
        let mut step = Step::default();
        for event in [ToolEvent::Down(pointer), ToolEvent::Up(pointer), ToolEvent::Move(pointer)] {
            step = step.then(self.advance(ctx, &event).0);
            if step.refused.is_some() {
                break;
            }
        }
        (step, self.tool.preview(ctx))
    }

    fn cursor(&self, start: P) -> P {
        self.last.or_else(|| self.tool.anchor()).unwrap_or(start)
    }

    /// ⌨️ Moves the keyboard cursor by `delta` metres from where the pointer last was (else the point the gesture hangs on, else `start`) and shows the tool's marks there, as a pointer move to the exact point would.
    pub fn nudge(&mut self, ctx: &mut ToolContext<'_>, delta: P, start: P) -> (Step, Preview) {
        let at = typed::resolve(&typed::Entry::Relative(delta), Some(self.cursor(start)), None).unwrap_or(start);
        self.advance(ctx, &ToolEvent::Move(Pointer { at, modifiers: Modifiers::default(), tolerance: typed::REACH }))
    }

    /// ⌨️ Clicks at the keyboard cursor, as the typed point `x, y` would.
    pub fn place(&mut self, ctx: &mut ToolContext<'_>, start: P) -> (Step, Preview) {
        let at = self.cursor(start);
        self.click(ctx, at)
    }
}''')
rep('''    fn session(&mut self, window: &str, window_kind: &str, utility: &str) -> &mut ToolSession {''', '''    /// ⌨️ Moves the keyboard cursor of the session of `window` by `delta`, or clicks at it when there is no delta.
    #[allow(clippy::too_many_arguments)]
    pub fn cursor(&mut self, window: &str, window_kind: &str, utility: &str, ctx: &mut ToolContext<'_>, delta: Option<P>, start: P) -> (Step, Preview) {
        let session = self.session(window, window_kind, utility);
        match delta {
            Some(delta) => session.nudge(ctx, delta, start),
            None => session.place(ctx, start),
        }
    }

    fn session(&mut self, window: &str, window_kind: &str, utility: &str) -> &mut ToolSession {''')
rep('''enum Feed<'a> {
    Event(ToolEvent),
    Line(&'a str),
}''', '''enum Feed<'a> {
    Event(ToolEvent),
    Line(&'a str),
    Cursor { delta: Option<P>, start: P },
}''')
rep('''                Feed::Line(line) => owner.enter(&window, &ctx.window_kind, &ctx.utility, &mut tool, line),''', '''                Feed::Line(line) => owner.enter(&window, &ctx.window_kind, &ctx.utility, &mut tool, line),
                Feed::Cursor { delta, start } => owner.cursor(&window, &ctx.window_kind, &ctx.utility, &mut tool, *delta, *start),''')
rep('''/// ⌨️ Keeps `line` as the entry typed''', '''/// ⌨️ The point a keyboard cursor starts from when the pointer never was in the window and the gesture hangs on nothing: the centre of the view (the plan and the section centre their camera, the world has no plan point).
pub fn cursor_start(ctx: &BimDispatchCtx) -> P {
    match ctx.window_kind.as_str() {
        plan::WINDOW_KIND_ID => [ctx.plan.viewport.x, -ctx.plan.viewport.y],
        section::WINDOW_KIND_ID => [ctx.section.viewport.x, -ctx.section.viewport.y],
        _ => [0.0, 0.0],
    }
}

/// ⌨️ Moves the keyboard cursor of the addressed window by `delta` metres, or clicks at it when `delta` is none, and answers the emit like [`run`]: the arrow keys move the cursor, the place key clicks, so every placement tool can be driven without a pointer or a typed line.
pub fn run_cursor(ctx: &mut BimDispatchCtx, doc: &ArtifactView<'_, ModelSnapshot>, delta: Option<P>) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    if ctx.gestures.is_none() {
        return Err(fault(RETAINED_ROUTE, "gesture commands are reachable only through their retained route"));
    }
    let Some((surface, _)) = surface_of(ctx, doc.snapshot) else { return Ok(Emit::default()) };
    let start = cursor_start(ctx);
    settle(ctx, doc, surface, Feed::Cursor { delta, start })
}

/// ⌨️ Keeps `line` as the entry typed''')
open(p + ".new", 'w', encoding='utf-8', newline='').write(s)
