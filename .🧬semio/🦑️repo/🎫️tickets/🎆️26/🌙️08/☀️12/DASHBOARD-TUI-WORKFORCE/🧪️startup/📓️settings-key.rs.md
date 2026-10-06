                            Key::Char('l') => {
                                dash.queue_preference(crate::preferences::Change { language: Some(if matches!(dash.locale, Locale::English) { "de" } else { "en" }.into()), ..Default::default() });
                                dash.leader = LeaderMode::Idle;
                                true
                            }
                            Key::Char('h') => { dash.show_home(&mut tui); dash.leader = LeaderMode::Idle; true }
                            Key::Char('p') => {
                                dash.show_home(&mut tui);
                                if let Some(index) = dash.windows.iter().position(|window| window.id == dash.focused) { dash.replace_view(&mut tui, index, "settings"); dash.refresh_wizard_for(&mut tui, index); }
                                dash.leader = LeaderMode::Idle; true
                            }
                            Key::Char('f') => { dash.refresh_inventory(); dash.leader = LeaderMode::Idle; true }
                            Key::Char('e') => { dash.cancel_inventory(); dash.leader = LeaderMode::Idle; true }
