# Specification

## Systems

### Validator

Go validator (`blnbo/go`) validates blnbo translations against Berlin Building Code (BauO Bln) rules.

- **staircase-located** (§35): Necessary staircases must be in separate stairwells. Exemptions: external staircase, building class 1/2, two-storey ≤200m² usage unit with escape routes.
- **building-height-limit** (§2): Building height must not exceed 21m (high-rise threshold).

## Mechanisms

- Translation JSON from semio-to-blnbo translator piped to stdin.
- Report JSON with rules/clauses/status written to stdout.
- Staircase evaluation: per-staircase clause evaluation with exemption short-circuit.
- Height validation: simple threshold comparison.

## Concepts

### Building Class (Gebäudeklasse)

Five classes per §2(3) BauO Bln based on height, usage units, and gross floor area.

### Necessary Staircase (Notwendige Treppe)

Staircase required for fire escape per §35 BauO Bln.

### Separate Stairwell (Eigener Treppenraum)

Enclosed stairwell required for necessary staircases under certain conditions.

### Usage Unit (Nutzungseinheit)

Functional unit within a building with its own gross floor area.
