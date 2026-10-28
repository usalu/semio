# Thermostat Zone Editing

The focused thermostat inspector now renders a localized thermal-zone selector. Its complete action payload carries the current zone and all setpoints, and the parser merges the edited zone exactly like the existing schedule controls. The reducer validates the referenced zone and emits the existing ChangeThermostatZone mutation only when the reference changes; central native mutation application retains the other fields.

A language-neutral fixture supplies the replacement and missing reference. Added native reducer and actual inspector-binding round-trip regressions. These still await the complete editor dependency graph compilation. Existing thermostat command construction tests now supply their actual zone.

The independent SQLite oracle passed: valid reassignment retains all unrelated fields; a missing foreign key refuses atomically. Run `python 🧪️thermostat-zone-oracle.py` from this ticket.
