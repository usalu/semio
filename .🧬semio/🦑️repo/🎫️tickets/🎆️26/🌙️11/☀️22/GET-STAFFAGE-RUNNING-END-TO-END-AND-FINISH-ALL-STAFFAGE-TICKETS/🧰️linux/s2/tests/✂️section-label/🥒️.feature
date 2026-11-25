# language: en
Feature: A section is named by its letters, the same in every language
  The cut-line marker prints the name of a section beside its arrows and the caption says "Section A-A" / "Schnitt A-A".
  A name that is a phrase, or that differs between the languages, has another width in each language; the marker then reaches out of the body by another amount and the whole drawing lies elsewhere on the paper in German than in English.
  THE REFERENCE is svgelements: it reads where the paths of the written SVG (body, marker and dimension lines) land in each language.

  Scenario Outline: The validator accepts section letters only
    Given a front section named "<en>" in English and "<de>" in German
    When the asset is validated
    Then the issue "section-label" is <reported>

    Examples:
      | en                         | de                 | reported     |
      | A-A                        | A-A                | not reported |
      | A                          | A                  | not reported |
      | B–B                        | B–B                | not reported |
      | Longitudinal crate section | Kistenlängsschnitt | reported     |
      | A-A                        | B-B                | reported     |
      | a-a                        | a-a                | reported     |
      | AA                         | AA                 | reported     |

  Scenario: The side elevation lands at the same place in both languages exactly when the section is named by letters
    Given the crate stack of the catalog at 1:50 with its front section
    When the side elevation is written as SVG in English and in German, once named "A-A" and once named by the phrase
    Then svgelements finds every path at the same paper position in both languages for "A-A"
    And at positions more than a millimetre apart for the phrase
