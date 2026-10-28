Feature: Three Agents Build Side By Side On A 16 GB Machine

  Scenario: An Agent Session With Nothing Running Gets Its First Job
    Given two agents whose runs book every core of a declared 8-core machine
    When a third agent asks for capacity and its memory fits
    Then it is admitted at once with one job, past the cores ceiling of our own runs
    And a second request of an agent that already runs still waits for cores

  Scenario: A Growth Short Of Cores Runs With The Jobs They Leave
    Given a run whose quiet reservation has to grow and whose whole request fits the memory
    When the cores our other runs book leave fewer jobs than it asked for
    Then it grows with the jobs the cores leave, at least one, and keeps the memory of its whole request
    And it says so in English or German, and grows to more jobs once cores free up

  Scenario: The Project Farm Never Makes A Run Wait
    Given a run whose graph construction asks for farm workers beside its own grant
    When other runs wait or arrive later
    Then the farm request is decided after all of them, never becomes a barrier and is given back when its workers are refused

  Scenario: A Nested Invocation Works With The Grown Grant
    Given an invocation that inherits a quiet grant from the command that started it
    When the grant grows before the invocation starts its tool
    Then the invocation's jobs, cores and plan are the grant's after the growth, never the quiet row's single job
