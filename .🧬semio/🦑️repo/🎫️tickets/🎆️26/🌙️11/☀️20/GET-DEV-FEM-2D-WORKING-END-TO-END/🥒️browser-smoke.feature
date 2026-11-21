Feature: FEM 2D analysis in the development app

  Scenario: The canvas paints a static analysis status
    Given a results text layer with an explicit catalog font and foreground color
    When the real Canvas2d renderer paints the layer
    Then the canvas displays the analysis status

  Scenario: Static results specify force and moment units
    Given a vertical reaction of 12 newtons and a rotational reaction of -40 newton metres
    Then the result labels read "Ty: 12 N" and "Rz: -40 N·m"

  Scenario: Editing the demo updates the solved deformation
    Given the FEM 2D development app is serving its real component
    When I open the bundled example in the browser
    Then the Example control shows "Demo"
    And the model and results canvases are visible
    And the results canvas shows a converged static analysis with deformation
    When I select node n1 in Artifact and change its X coordinate from 0 to 1.5 in Inspection
    Then the Artifact panel shows the modified coordinate
    And the results canvas shows a converged static analysis with updated deformation
    When I select the demo's Live Load in the Results panel
    Then the results canvas shows a positive 12000 N vertical reaction

  Scenario: The demo's mounted analysis displays positive mode results
    Given the bundled example has converged for Live Load
    When I select Modal in the Results panel
    Then the first mode shows a positive frequency in hertz and a converged mode shape
    When I select Buckling for the same Live Load source
    Then the first buckling mode shows a positive factor and a converged mode shape
