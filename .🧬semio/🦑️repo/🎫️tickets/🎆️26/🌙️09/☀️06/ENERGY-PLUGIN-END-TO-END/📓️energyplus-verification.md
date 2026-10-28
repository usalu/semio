# EnergyPlus Exchange Verification

Official EnergyPlus 25.2.0 and its shipped epJSON schema validated the following committed inputs.

- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏛️bestest-600/⚡️model.epJSON`: successful; schema valid.
  - Zone Ideal Loads Supply Air Total Cooling Energy: 6044.058559 kWh.
  - Zone Ideal Loads Supply Air Total Heating Energy: 4324.744191 kWh.
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏛️bestest-600FF/⚡️model.epJSON`: successful; schema valid.
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏛️bestest-900/⚡️model.epJSON`: successful; schema valid.
  - Zone Ideal Loads Supply Air Total Cooling Energy: 2498.161384 kWh.
  - Zone Ideal Loads Supply Air Total Heating Energy: 1661.137877 kWh.
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏛️bestest-900FF/⚡️model.epJSON`: successful; schema valid.
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/⚡️idf/🧪️tests/1ZoneUncontrolled.idf`: successful; schema valid.

These checks prove the committed exchanges are accepted by EnergyPlus. They do not prove numerical agreement of the native simulation or browser integration.
