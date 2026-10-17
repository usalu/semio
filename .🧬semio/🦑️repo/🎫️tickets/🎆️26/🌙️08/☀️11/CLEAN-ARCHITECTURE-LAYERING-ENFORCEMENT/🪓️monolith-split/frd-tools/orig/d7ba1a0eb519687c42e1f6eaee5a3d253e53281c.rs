use super::*;

#[test]
fn microsecond_clock_installation_is_exact_and_repeatable() {
    fn browser() -> Option<u64> {
        Some(500)
    }
    fn foreign() -> Option<u64> {
        Some(900)
    }
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../🧵️job/⏱️budget/🕰️clock.json")).unwrap();
    for law in fixture["installation"].as_array().unwrap() {
        let authority = OnceLock::new();
        let clock = |name: &str| if name == "browser" { browser as fn() -> Option<u64> } else { foreign as fn() -> Option<u64> };
        if let Some(current) = law["current"].as_str() {
            assert!(authority.set(clock(current)).is_ok());
        }
        let requested = clock(law["requested"].as_str().unwrap());
        assert_eq!(install_exact_clock(&authority, requested).is_ok(), law["accepted"].as_bool().unwrap());
        assert!(std::ptr::fn_addr_eq(*authority.get().unwrap(), clock(law["retained"].as_str().unwrap())));
    }
}

#[test]
fn microsecond_watchdog_boundary_is_strictly_below_eight_ms() {
    let fixture: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../🧵️job/⏱️budget/🕰️clock.json")).unwrap();
    for law in fixture["watchdog"].as_array().unwrap() {
        let elapsed = law["elapsedMicroseconds"].as_str().unwrap().parse().unwrap();
        assert_eq!(interactive_step_contract_violated(elapsed), law["violated"].as_bool().unwrap());
    }
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
