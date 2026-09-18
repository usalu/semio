@capability-wfc-bitmap-1-mount
@comparison-ordered-json-v1
Feature: The bitmap subset mounts the identities, windows and examples it declares
  The committed `../../🧫️fixtures/🧩️mount-contract/🔣️.json` is the language-agnostic statement of
  what this subset mounts. `🦀️.rs` beside this file is its Rust half — it reads the real manifests
  the plugin builder produces — and `🐍️.py` is a second reader that links nothing of this repository
  and checks the same statement against the kind directories on disk.

  Scenario: The declared surface ids are the canonical ones
    Given the committed mount contract
    Then the editor app id is s.wfc.bitmap@1/*#editor
    And the viewer app id is s.wfc.bitmap@1/*#viewer
    And the OS artifact kind id is 2d.wfcbitmap
    And the inference tool id is s.wfc.bitmap.solve

  Scenario: Both surfaces declare exactly the committed window kinds
    Given the committed mount contract
    Then the editor declares wfc-bitmap-input and wfc-bitmap-output, in that order
    And the viewer declares the same two window kinds, in the same order

  Scenario: The mutation vocabulary on disk is the declared one
    Given the committed mount contract
    Then every kind directory under 🧬️schema/🧬️mutations declares a semantic kind the contract lists
    And the contract lists no kind that has no directory

  Scenario: Every bundled example is real and localized
    Given the committed mount contract
    Then every example declares a sample with a positive extent and at least two palette colours
    And every example carries an English and a German label
