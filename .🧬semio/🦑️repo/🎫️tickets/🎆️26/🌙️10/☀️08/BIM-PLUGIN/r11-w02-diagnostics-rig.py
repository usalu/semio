import sys
E = sys.argv[1]
def patch(p, pairs):
    s = open(p, encoding='utf-8', newline='').read()
    for old, new in pairs:
        assert s.count(old) == 1, (p, old, s.count(old))
        s = s.replace(old, new)
    open(p + ".new", 'w', encoding='utf-8', newline='').write(s)
patch(E + "/🧵️gestures/🦀️.rs", [("    fn cursor(&self, start: P) -> P {", "    /// ⌨️ Where the keyboard cursor stands: where the pointer last was, else the point the gesture hangs on, else `start`.\n    pub fn cursor(&self, start: P) -> P {")])
patch(E + "/🧵️gestures/🧪️tests/🔬️unit/🦀️.rs", [("""        /// ⌨️ Types one line into the entry field the way a submit does;""", """        /// ⌨️ Moves the keyboard cursor by `delta` metres from `start` (or from where the pointer last was); every mutation of the answer is applied to the rig's model.
        pub fn nudge(&mut self, delta: [f64; 2], start: [f64; 2]) -> Step {
            self.drive(|session, context| session.nudge(context, delta, start))
        }

        /// ⌨️ Clicks at the keyboard cursor; every mutation of the answer is applied to the rig's model.
        pub fn place(&mut self, start: [f64; 2]) -> Step {
            self.drive(|session, context| session.place(context, start))
        }

        /// ⌨️ Types one line into the entry field the way a submit does;""")])
