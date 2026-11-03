@capability-norm-check-stations @oracle-python @comparison-closed-form
Feature: Check stations carry utilisation along members and over plates
  A check may sample its utilisation at positions of the entity (`[s]`, `[x, y]` or `[x, y, z]`). The governing utilisation always equals the maximum over the stations, and the fixture is validated by the third-party Ajv validator and an independent Python oracle.

  Further specification assertions, asserted by the Rust unit tests of the crate and not by the platform adapters:
  - The governing utilisation is the maximum over the stations: Given a member check sampled at five positions along its length When the check is built Then its utilisation equals the largest station utilisation And a station above 1.0 makes the check fail
  - Stations are bounded by the schema: Given the committed check report fixture with stations Then a position with four coordinates is rejected And a negative station utilisation is rejected
  - Not applicable checks carry no stations: Given a check marked not applicable after stations were attached When the check is built Then it has no stations and zero utilisation
