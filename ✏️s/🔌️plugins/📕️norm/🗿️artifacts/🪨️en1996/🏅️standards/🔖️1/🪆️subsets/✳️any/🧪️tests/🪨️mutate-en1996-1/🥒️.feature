Feature: EN 1996 masonry mutations
  Scenario: eight semantic mutations cover the rebuilt subject
    Given a compliant clay wall snapshot
    Then change-annex, insert-wall, remove-wall, change-wall-thickness, change-wall-height, change-unit-fb, change-mortar-class, and change-support-sides apply
