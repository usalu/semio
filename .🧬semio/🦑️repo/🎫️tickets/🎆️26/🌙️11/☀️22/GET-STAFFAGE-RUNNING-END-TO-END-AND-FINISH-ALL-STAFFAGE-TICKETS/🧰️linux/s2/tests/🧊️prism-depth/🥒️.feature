# language: en
Feature: A swept solid is ranked by the depth of its own prism
  The paint order of a pose follows the physical depth of the shapes: the highest point in the plan, the foremost in the front, the smallest lateral position in the side elevation.
  An extruded shape is a prism: its profile lies in the (u, v) plane for a sweep along w, in the (u, w) plane for a sweep along v and in the (v, w) plane for a sweep along u, and it reaches from z0 to z1 (tapering to zTip at the bone tip).
  Its depth is the depth of the nearest corner of that prism, whatever the sweep and however the bone is turned.
  THE REFERENCE is numpy: it places every profile point at both ends of the sweep in the frame of the bone, by the definition of the three sweeps, and reads the nearest coordinate.

  Scenario Outline: The depth of a box drawn as a prism equals the nearest corner numpy finds
    Given a 6 mm case bottom on the left thigh drawn as a prism swept along <sweep>
    When the figure stands, strides, spreads the thigh by -30 degrees and rolls it by -91 degrees
    Then in the side elevation, the front and the plan the depth of the shape equals the nearest corner of the prism within a nanometre

    Examples:
      | sweep                 |
      | w                     |
      | v                     |
      | u                     |
      | v, tapering to a tip  |

  Scenario: A rolled case bottom is not ranked through the case
    Given the case bottom swept along v and the thigh rolled by -91 degrees
    When the side elevation is ranked
    Then the depth of the bottom is its own lateral face, not the plane of the bone 0.2 m away
