Feature: Native Dashboard Keyboard Controls
  Scenario: A portable control key opens optional settings
    Given the shared native keyboard input vectors
    When the terminal receives Ctrl B followed by p
    Then Ctrl B opens the controls and p opens optional settings
    And the input projection matches Node readline independently

  Scenario: Unicode input survives terminal decoding
    Given the same shared vectors contain UTF-8 Ü
    When Rust and Node independently decode the input
    Then both project the same character without a control modifier

  Scenario: Windows owns UTF-8 code pages only while attached
    Given the native terminal acquires UTF-8 input and output code pages
    When cleanup restores one page and temporarily fails to restore the other
    Then only the remaining owned page is retried
    And an actual native view renders box drawing and German labels correctly
