
use super::*;

#[test]
fn default_snapshot_parses_the_bundled_example() {
    assert!(!default_snapshot().fixture.widgets.is_empty());
}
