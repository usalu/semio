Feature: On Windows the runtime never runs from the tools partition
  Scenario: The user cache wins and a tools-partition copy is adopted
    Given a runtime installed in the tools partition and in the user cache
    Then Windows resolves the user cache while every other platform resolves the tools partition
    When only the tools partition holds the runtime
    Then it is copied into the user cache once, with no download, and the copy runs
    And peers that adopt at the same time end with one valid copy and no staging leftovers
    And a refused adoption keeps the tools-partition copy running
    And on a Windows host a running image blocks renaming its directory, the adopted copy does not
