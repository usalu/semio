Feature: A Run Admitted With Fewer Jobs Grows When Memory Frees Up

  Scenario: Freed Memory Grows The Jobs Of A Running Build
    Given a build admitted with 1 of 8 jobs because the host could not carry more
    When memory frees up and stays free for consecutive heartbeats after its last reservation was observed
    Then its grant grows to the most jobs whose memory fits with the memory of one more job left free
    And it says so in English or German with the jobs and the reservation before and after
    And every process of the run that starts work afterwards plans with the grown jobs

  Scenario: A Growth Takes Nothing From A Waiting Run
    Given a build that runs with fewer jobs than it asked for
    And another run of ours waits for memory
    When memory frees up
    Then the waiting run is served first and the build grows only once nothing of ours waits

  Scenario: Memory That Frees Up For A Moment Never Grows A Run
    Given a build that runs with fewer jobs than it asked for
    When free memory alternates between enough and too little from one heartbeat to the next
    Then the build keeps its jobs and never shrinks or grows back and forth

  Scenario: A Compile Takes The Cores Its Grant Carries
    Given a compile inside a run admitted with one job whose memory carries several compile units at the footprint the compiler broker measured
    When the compile plans its cores
    Then it takes the cores that memory carries instead of one, within the share of the host
    And the compiler broker never holds more memory than the grant gives the compile
