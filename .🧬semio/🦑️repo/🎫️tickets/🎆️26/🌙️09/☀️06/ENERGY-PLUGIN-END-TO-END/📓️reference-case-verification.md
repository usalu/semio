# Case-Insensitive Energy References

The shared reference-case.json law uppercases zone, construction, host-surface and layer reference names without changing object names. Official EnergyPlus 25.2 successfully runs both documents and reports exactly equal annual heating and cooling joules. This independently validates the native importer regression expectation; full importer integration tests still require the shared store build.
