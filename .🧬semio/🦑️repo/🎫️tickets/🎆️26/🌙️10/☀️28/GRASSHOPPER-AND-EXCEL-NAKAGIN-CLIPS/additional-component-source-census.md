# Additional Component Source Census

A read-only check of components that are difficult to distinguish in small gallery views found explicit lift-cabin handrail, stair rail beams and endpoint posts in the current shared physical source. This rejects an apparent omission inferred only from reduced image scale. It is source-presence evidence, not an additional runtime/contact test or claim that every rail is visible through opaque walls. Existing physical/structural runtime proofs and current full native/image gates remain unchanged. No source edit or extra geometry run was needed.

```json
{
  "path": "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py",
  "raw_sha256": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
  "matches": [
    {
      "line": 2668,
      "source": "add(\"reference-cabin-handrail\", \"lift_cabin_handrail\", _beam((x0 + .09, y0 + .07, z + .92), (x0 + .09, y1 - .07, z + .92), .025), \"metal\")"
    },
    {
      "line": 2746,
      "source": "parts.append(NamedPart(f\"stair-rail-{flight}\", \"stair_rail\", place(rail, plane), \"metal\"))"
    },
    {
      "line": 2749,
      "source": "parts.append(NamedPart(f\"stair-rail-post-{flight}-{end}\", \"stair_rail\", place(post, plane), \"metal\"))"
    }
  ]
}
```
