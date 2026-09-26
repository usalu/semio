        /// ⌨️ One long typing run (fixture `🧫️fixtures/⌨️typing-run/🔣️.json`: ≥ 1000 typed characters with pauses, caret moves back
        /// into the run and Backspace/Delete corrections) as the full texts a text-editor host delivers, one per changed key.
        pub struct TypingRun {
            pub initial: String,
            pub texts: Vec<String>,
            pub expected: String,
        }

        /// ⌨️ Replays the typing-run fixture under the caret-relative text-input model (typing inserts at the caret,
        /// Backspace/Delete remove one character, arrows move one character, Home/End reach the line edges, Enter inserts a line
        /// break) — the model the editor's own text-input law and Chromium's native textarea answer identically.
        pub fn typing_run() -> TypingRun {
            let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/⌨️typing-run/🔣️.json")).expect("typing-run fixture");
            let initial = fixture["initial"].as_str().expect("typing-run initial").to_string();
            let mut text: Vec<char> = initial.chars().collect();
            let mut caret = text.len();
            let mut texts = Vec::new();
            for key in fixture["keys"].as_array().expect("typing-run keys") {
                let before = text.clone();
                match key.as_str().expect("typing-run key") {
                    "pause" => {}
                    "Backspace" => {
                        if caret > 0 {
                            caret -= 1;
                            text.remove(caret);
                        }
                    }
                    "Delete" => {
                        if caret < text.len() {
                            text.remove(caret);
                        }
                    }
                    "ArrowLeft" => caret = caret.saturating_sub(1),
                    "ArrowRight" => caret = (caret + 1).min(text.len()),
                    "Home" => caret = text[..caret].iter().rposition(|ch| *ch == '\n').map_or(0, |index| index + 1),
                    "End" => caret = text[caret..].iter().position(|ch| *ch == '\n').map_or(text.len(), |index| caret + index),
                    "Enter" => {
                        text.insert(caret, '\n');
                        caret += 1;
                    }
                    typed => {
                        for ch in typed.chars() {
                            text.insert(caret, ch);
                            caret += 1;
                        }
                    }
                }
                if text != before {
                    texts.push(text.iter().collect());
                }
            }
            let expected = fixture["expect"]["text"].as_str().expect("typing-run expected text").to_string();
            assert_eq!(texts.last(), Some(&expected), "the typing-run model reaches the fixture's expected text");
            TypingRun { initial, texts, expected }
        }

