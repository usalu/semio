Feature: A run that follows a finished run of its class asks for the small quiet grant
  A fully cached run executes nothing, so it must not wait in the machine-wide queue for the full grant of a real build

  Scenario: The hot no-op of gate-g row 4c
    Given the class of "run @teaching/architecture-course:build" learned 200% of its default from 12 earlier runs, the last two executed the planner and held at most 2.8 GB
    And the earlier run finished 90 seconds ago
    When the same build is started again
    Then the door forecasts it quiet because the earlier run of its class left its results cached
    And it reserves the small quiet grant instead of 7168 MB
    And the full request is only asked for when work has to run, with the queue position the run arrived with

  Scenario: Raised pressure does not hold a quiet grant
    Given a 16 GB profile with 5686 MB spare above the floor, raised memory pressure and nothing running
    Then the full request of 7168 MB waits for memory
    And the quiet request of 2048 MB is admitted at once
    And critical pressure still holds it

  Scenario: The window ends
    Given no run of the class finished within the follow-up window and fewer than two quiet runs are known
    Then the door asks for the full grant

  Scenario: A multi-worker target declares its envelope
    Given the policy declares an envelope of 4 cores and 6144 MB for targets named "test-e2e"
    When "run @teaching/architecture-course:test-e2e" is admitted as one job
    Then the grant carries 4 cores and 6144 MB
    And the end-to-end stack planner counts on that share and plans more than one stack

  Scenario: A declared machine
    Given SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES is "16384" and SEMIO_HOST_CAPACITY_CORES is "8" on a 32 GB host with 10 cores
    Then the capacity gate, the worker policy, the Bazel server plan and the test runners plan for 16384 MB and 8 cores
    And a malformed value means the real host and a value above the real host never grows it

  Scenario: The shared sample file
    Given a process without a declaration and a process that declares 16384 MB share one gate directory
    Then each process reuses only a sample taken for its own declaration
