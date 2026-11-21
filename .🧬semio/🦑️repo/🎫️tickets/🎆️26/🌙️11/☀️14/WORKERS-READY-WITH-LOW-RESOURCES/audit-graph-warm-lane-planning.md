# Warm Owner Lane Planning

The live whole-graph reader entered its warm wave with 4,631 available owner record names and nine children. Each comparison among equal-cost owners used to hash both record keys. A 4,631-owner portable fixture reproduced 96,970 key reads while preserving a reference lane plan. The updated planner computes the ordering hash once per owner and still selects the lightest lane, preserves the cost and name tie order, and schedules the lowest-cost owner first in each lane. The SQLite oracle orders the same input independently. The new fixture requires at most 4,631 key reads.

The two-file warm suite passed 19 tests and 124 assertions. An 11-pair Bun comparison on the same fixture produced equal lane arrays and median planning times of 173.862 milliseconds and 9.910 milliseconds. It measured lane planning only. It did not time farm workers or complete a graph. The executable comparison is `measure-warm-lanes.ts`.
