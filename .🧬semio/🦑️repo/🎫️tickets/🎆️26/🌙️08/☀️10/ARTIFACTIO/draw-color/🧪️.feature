Feature: Controlled Drawing color and dash text boundaries
  Scenario: Hexadecimal colors admit normalized typed RGBA
    Given an exact #RGB or #RRGGBB spelling and finite unit alpha
    When explicit controlled color IO admits the text
    Then components agree with an independent color library
    And publication uses lowercase six-digit hexadecimal RGB
  Scenario: Malformed UTF-8 color inputs refuse before publication
    Given a short, long, non-hexadecimal or non-ASCII color spelling
    When explicit color IO reads the input
    Then admission refuses without a panic or a fallback color
  Scenario: Dash text yields typed nonnegative finite samples
    Given the authored dash spelling corpus
    When explicit controlled dash IO admits the text
    Then semantic reducers receive samples or no pattern
    And cancellation and owned allocation limits refuse before publication
