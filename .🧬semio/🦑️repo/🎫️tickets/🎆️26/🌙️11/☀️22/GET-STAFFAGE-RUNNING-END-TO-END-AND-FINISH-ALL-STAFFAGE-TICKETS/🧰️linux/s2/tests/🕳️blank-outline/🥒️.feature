# language: en
Feature: A shape drawn as an unfilled outline is drawn
  The matrix reports `blank-shapes` when a scene draws none of the shapes of its view.
  A shape leaves ink as the fill of a filled closed shape or as the outline runs of its pen; a symbol whose block is an unfilled outline is drawn, and a shape whose pen is zero is not.
  THE REFERENCE is sharp (libvips with librsvg): it rasterises the SVG of the path items of the shapes and counts the pixels that carry ink.

  Scenario Outline: The drawn shapes of a view are the ones librsvg puts ink for
    Given the symbol "gardener-block" of the kneeling gardener at 1:<scale>, one closed unfilled outline per view with a body pen of 0.18 mm
    When the <view> is drawn in the mode "<mode>"
    Then the shapes of the view count as drawn, and librsvg puts ink for their path items
    And with the strokes of those path items set to a width of zero they do not count as drawn, and librsvg puts no ink

    Examples:
      | scale | view      | mode        |
      | 200   | elevation | dark-line   |
      | 200   | front     | light-solid |
      | 200   | plan      | thermal     |
      | 500   | elevation | light-line  |
      | 500   | plan      | dark-solid  |

  Scenario: The reason of a blank view names the pen of the symbol
    Given the same symbol
    When the reason of a blank view is written
    Then it states the body pen of the symbol, 0.18 mm, and not "none"
