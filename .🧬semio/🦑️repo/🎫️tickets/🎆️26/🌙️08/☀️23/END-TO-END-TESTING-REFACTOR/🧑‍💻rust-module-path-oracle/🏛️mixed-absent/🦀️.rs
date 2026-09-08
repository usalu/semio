#[path = "named-scope"]
mod first {
    #[path = "../reassigned-scope"]
    mod second {
        #[cfg(test)]
        #[path = "🧪️tests/🔬️mixed-absent/🦀️.rs"]
        mod canonical;
    }
}
