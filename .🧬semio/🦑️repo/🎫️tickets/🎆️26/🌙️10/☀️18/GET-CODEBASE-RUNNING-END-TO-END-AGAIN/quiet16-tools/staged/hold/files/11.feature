Feature: The Real Actions Of A Build Run Only As Wide As Its Grant

  Scenario: The First Real Action Waits For The Growth
    Given a run admitted with a small quiet grant whose Bazel was started with the full request's jobs
    When its first action that really runs is born
    Then the birth waits in the fair queue until the grant has grown, with the run's original arrival

  Scenario: A Partial Growth Never Runs At Full Width
    Given a grant that grew to fewer jobs than Bazel was started with
    When more actions are born than the grant holds jobs
    Then only as many run as the grant holds jobs, the others wait first come first served
    And they start when an action ends or when the grant grows further

  Scenario: The Gate Equals An Independent Model
    Given seeded traces of births, ends, cancellations, changing jobs and rechecks
    When the gate replays them
    Then who runs, who waits and who was refused equal the Python model after every event
