Feature: Animated pictures retain their actual surface colors across cached frames
  Scenario: A painted element changes color without changing its outline
    Given a red square centered on a 64 by 64 picture
    When its fill becomes blue
    Then the next rendered center pixel is blue

  Scenario: Identical pictures recur at different times
    Given the same camera, geometry, colors, size and frame rate
    When an identical picture plays for four frames
    Then all four frames appear in the exported movie
    And a cached single frame cannot shorten the run
