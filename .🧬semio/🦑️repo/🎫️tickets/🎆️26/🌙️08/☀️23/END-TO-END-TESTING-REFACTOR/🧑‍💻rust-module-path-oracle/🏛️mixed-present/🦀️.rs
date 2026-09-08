#[path = "named-scope"]
mod first {
    #[path = "../reassigned-scope"]
    mod second {
        #[cfg(test)]
        #[path = "🧪️tests/🔬️mixed-present/🦀️.rs"]
        mod canonical;
    }
}
