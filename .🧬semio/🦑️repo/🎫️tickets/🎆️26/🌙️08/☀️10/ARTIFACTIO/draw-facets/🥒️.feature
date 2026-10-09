Feature: Semantic Draw Image Assets and Relational Samples
  Scenario: Decode physical PNG before adopting logical samples
    Given an encoded PNG input owned by image IO
    When the admitted drawing image is independently observed with pngjs
    Then dimensions and each ordered RGBA tuple equal the independent pixels
    And the canonical document stores no MIME or encoded source
  Scenario: Preserve ordered components through independent SQLite
    Given a canonical image with two tuples containing zero,255 and alpha128
    When the snapshot passes through its declared relational IO
    Then33 authored tables include ordered asset sample rows
    And an independent SQL component edit changes the restored sample
    And orphan rows and dimensional sample mismatches are refused
  Scenario: Keep decoder authority out of semantic scenes
    Given admitted images with intrinsic RGBA component bytes
    Then scene budgets own maxSourceBytes and maxAdmittedPixels
    And physical maxBytes and maxChunks belong only to image IO
  Scenario: Retire the actual image owners
    Given a prepared plan with one admitted image
    Then its asset retains an identity and admitted image owner
    And independent owner census counts three asset owners
