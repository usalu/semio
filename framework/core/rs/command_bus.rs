//! 🎯 Command routing between renderer and app controllers.

use serde_json::Value;
use std::collections::HashMap;

pub trait CommandHandler: Send {
    fn id(&self) -> &str;
    fn handle(&mut self, command: &str, args: Option<&Value>) -> Vec<String>;
}

pub struct CommandBus {
    controllers: HashMap<String, Box<dyn CommandHandler>>,
}

impl Default for CommandBus {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandBus {
    pub fn new() -> Self {
        Self {
            controllers: HashMap::new(),
        }
    }

    pub fn register(&mut self, handler: Box<dyn CommandHandler>) {
        let id = handler.id().to_string();
        self.controllers.insert(id, handler);
    }

    pub fn unregister(&mut self, controller_id: &str) {
        self.controllers.remove(controller_id);
    }

    pub fn dispatch(&mut self, controller_id: &str, command: &str, args: Option<&Value>) -> Vec<String> {
        self.controllers
            .get_mut(controller_id)
            .map(|handler| handler.handle(command, args))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoHandler {
        id: String,
    }

    impl CommandHandler for EchoHandler {
        fn id(&self) -> &str {
            &self.id
        }

        fn handle(&mut self, command: &str, _args: Option<&Value>) -> Vec<String> {
            vec![format!("{command}:ok")]
        }
    }

    #[test]
    fn dispatches_to_registered_handler() {
        let mut bus = CommandBus::new();
        bus.register(Box::new(EchoHandler { id: "app".into() }));
        let ops = bus.dispatch("app", "ping", None);
        assert_eq!(ops, vec!["ping:ok"]);
    }
}
