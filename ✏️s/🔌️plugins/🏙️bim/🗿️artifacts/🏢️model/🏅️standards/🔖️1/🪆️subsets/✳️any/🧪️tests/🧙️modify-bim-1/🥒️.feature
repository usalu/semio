@capability-bim-1-modify
@oracle-bim-1-shapely-geometry
@comparison-floating-point-v1
Feature: Copy, mirror, array, offset, trim, extend, align and split authored geometry and audit the numbers with shapely
  The modify toolset of `s.bim.model@1` (`copy-elements`, `mirror-elements`, `array-elements`, `align-elements`, `offset-wall`, `trim-extend-wall`, `split-slab`,
  `split-beam`, `set-wall-end-join`) acts on authored parameters only. Each mutation computes its numbers in the shared kernel (`🧬️mutations/🧙️modify`) from the
  payload and the base snapshot: the ids of copies are minted from the prefix, the copy number and the position of the source, never from a clock or a counter, and
  the inverse of a copy is one `delete-elements` of the minted ids while the inverse of an in-place mirror restores absolute base placements. The oracle is `🐍️.py`
  in this directory. It recomputes every case with `shapely` 2 (GEOS) and `numpy`: `affinity.translate`, `rotate` and `affine_transform` with the reflection matrix
  for the maps, `LineString.offset_curve` and concentric circles for the offsets, infinite-line and full-circle intersections for trim and extend, half-plane
  intersections for the halves of a cut polygon, and mitred or butted offset carriers for the footprints of joined wall ends. It writes the expectations into
  `🧫️fixtures/🧙️modify/🔣️.json`, which the Rust subject replays through its own kernel (`🧙️modify/🧪️tests/🔬️unit`). The tools that propose these mutations
  (`✏️editor/🧵️gestures/{👯️duplicate,✂️reshape,🔪️split}`) preview every step in the window transient, so the plan and the 3D window show the result before the
  closing click, and write nothing until it.

  @id-modify-maps
  @level-exhaustive
  @mode-differential
  Scenario Outline: The <family> map sends the committed points and directions where shapely sends them
    Given the committed modify cases shared://🧫️fixtures/🧙️modify/🔣️.json
    When the <family> map of case <case> is applied to its points and directions
    Then every image equals the committed image within 1e-9 metres and every turned direction within 1e-9 radians
    Examples:
      | family    | case |
      | translate | 0    |
      | rotate    | 0    |
      | rotate    | 1    |
      | mirror    | 0    |
      | mirror    | 1    |
      | mirror    | 2    |

  @id-modify-loops
  @level-exhaustive
  @mode-differential
  Scenario Outline: A mirrored bulged loop keeps its area, runs counter-clockwise and has its centroid mirrored (case <case>)
    Given the committed modify cases shared://🧫️fixtures/🧙️modify/🔣️.json
    When the loop of case <case> is mirrored about its line
    Then the area of the image equals the committed image area within 1e-5 and its centroid the committed image centroid
    And the image runs counter-clockwise
    Examples:
      | case |
      | 0    |
      | 1    |

  @id-modify-offsets
  @level-exhaustive
  @mode-differential
  Scenario Outline: The wall axis of offset case <case> moves to the parallel curve shapely computes
    Given the committed modify cases shared://🧫️fixtures/🧙️modify/🔣️.json
    When the axis of offset case <case> is offset by its signed distance
    Then the image axis equals the committed image within 1e-5
    Examples:
      | case |
      | 0    |
      | 1    |
      | 2    |
      | 3    |
      | 4    |

  @id-modify-trims
  @level-exhaustive
  @mode-differential
  Scenario Outline: The end of trim case <case> lands on the intersection with the carrier of its target
    Given the committed modify cases shared://🧫️fixtures/🧙️modify/🔣️.json
    When the named end of the axis of trim case <case> is moved onto the carrier of its target
    Then the moved end equals the committed point within 1e-9 metres
    Examples:
      | case |
      | 0    |
      | 1    |
      | 2    |
      | 3    |
      | 4    |
      | 5    |

  @id-modify-splits
  @level-exhaustive
  @mode-differential
  Scenario Outline: A cut along the line of split case <case> leaves the halves shapely intersects
    Given the committed modify cases shared://🧫️fixtures/🧙️modify/🔣️.json
    When the loop of split case <case> is cut along its line
    Then the left and the right half have the committed areas and centroids within 1e-9 metres
    Examples:
      | case |
      | 0    |
      | 1    |
      | 2    |
      | 3    |

  @id-modify-arrays
  @level-exhaustive
  @mode-differential
  Scenario Outline: The copies of radial array case <case> sit at the k-th turn of the point about the centre
    Given the committed modify cases shared://🧫️fixtures/🧙️modify/🔣️.json
    When the point of radial array case <case> is turned by the k-th multiple of its step for k up to its count
    Then the images equal the committed images within 1e-9 metres
    Examples:
      | case |
      | 0    |
      | 1    |

  @id-modify-joins
  @level-exhaustive
  @mode-differential
  Scenario Outline: The wall ends of join case <case> resolve to the footprints of the authored join preference
    Given the committed modify cases shared://🧫️fixtures/🧙️modify/🔣️.json
    When the walls of join case <case> are laid out with their authored end joins
    Then every footprint equals the committed footprint within 1e-9 metres
    Examples:
      | case |
      | 0    |
      | 1    |
      | 2    |
      | 3    |
      | 4    |
      | 5    |
      | 6    |
      | 7    |

  @id-modify-gestures
  @level-quick
  @mode-scenario
  Scenario: Every modify tool shows its result live and writes it with its closing click
    Given a plan window over the demo room with the copy, mirror, array, radial array, offset, trim, extend, align and split tool armed in turn
    When the pointer moves over the elements and the points of the gesture are clicked up to, but not including, the closing click
    Then the window transient holds the marks of the gesture (outlines of what will be acted on, ghosts of the result, guide lines and labels) at every step
    And the 3D window paints the same marks at the floor of the storey
    And the document is unchanged until the closing click
    And the closing click writes exactly one mutation, one history row, and selects the created elements

  @id-modify-hotkeys
  @level-quick
  @mode-scenario
  Scenario: Every modify tool has its own hotkey and no key is bound twice
    Given the editor manifest with its commands, its utilities and the keys of a gesture in progress
    When the keybindings are listed
    Then copy is K, mirror is Shift+K, array is Y, radial array is Shift+Y, offset is F, trim is X, extend is Shift+X, align is Z and split is Shift+Z
    And the wall flip is Shift+F so that it does not take the key of the offset tool
    And no key is bound to two actions
