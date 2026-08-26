fn main() {
    let definition = semio_s_plugin_puzzle::editor::puzzle3d::create_puzzle3d_app();
    println!("[DEBUG] id={} controller={}", definition.id, definition.controller_id);
    for command in &definition.commands {
        println!("[DEBUG] app-command {} {:?}", command.id, command.semantics.execution.interactive_job);
    }
    for mode in definition.modes.iter() {
        for command in &mode.commands {
            println!("[DEBUG] mode-command {}:{} {:?}", mode.id, command.id, command.semantics.execution.interactive_job);
        }
    }
    for window in definition.window_kinds.iter() {
        for action in &window.actions {
            println!("[DEBUG] window-action {}:{} {:?}", window.id, action.id, action.semantics.execution.interactive_job);
        }
    }
}
