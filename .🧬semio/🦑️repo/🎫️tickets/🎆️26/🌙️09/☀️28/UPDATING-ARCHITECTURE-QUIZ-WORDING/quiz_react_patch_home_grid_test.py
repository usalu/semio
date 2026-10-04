import io, sys
p = sys.argv[1]
s = io.open(p, encoding="utf-8", newline="").read()
reps = [
("""function row(rank: number, tag: string): LeaderboardRow {
  return { rank, tag, identity: { kind: "pseudonym", handle: `Learner ${rank}` }, total: 1000 - rank * 10, reachedAt: rank, best: {}, badges: [], runs: 1, lastActivity: rank };
}

function rows(ranks: number, mine: number | null): readonly LeaderboardRow[] {
  return Array.from({ length: ranks }, (_, index) => row(index + 1, index + 1 === mine ? learnerTag(LEARNER) : `${String(index + 1).padStart(8, "0")}`));
}""",
"""function row(rank: number, tag: string, handle: string): LeaderboardRow {
  return { rank, tag, identity: { kind: "pseudonym", handle }, total: 1000 - rank * 10, reachedAt: rank, best: {}, badges: [], runs: 1, lastActivity: rank };
}

function rows(ranks: number, mine: number | null): readonly LeaderboardRow[] {
  return Array.from({ length: ranks }, (_, index) => (index + 1 === mine ? row(index + 1, learnerTag(LEARNER), "Ada") : row(index + 1, String(index + 1).padStart(8, "0"), `Learner ${index + 1}`)));
}"""),
("""        const feature = query.mediaQueries[0]?.condition.value;
        if (feature?.name !== "width" || feature.operator !== "greater-than-equal" || feature.value.value.value.unit !== "px") throw new Error(`unexpected media query ${JSON.stringify(query)}`);
        collect(rule.value.rules ?? [], feature.value.value.value.value);
        continue;""",
"""        const feature = query.mediaQueries[0]?.condition.value;
        if (feature?.name === "width") {
          if (feature.operator !== "greater-than-equal" || feature.value.value.value.unit !== "px") throw new Error(`unexpected width query ${JSON.stringify(query)}`);
          collect(rule.value.rules ?? [], feature.value.value.value.value);
        } else {
          const before = found.length;
          collect(rule.value.rules ?? [], Number.POSITIVE_INFINITY);
          if (found.length !== before) throw new Error(`grid rules under a media query that is not about width: ${JSON.stringify(query)}`);
        }
        continue;"""),
]
for a, b in reps:
    if s.count(a) != 1:
        sys.exit(f"anchor {s.count(a)}: {a[:60]!r}")
    s = s.replace(a, b)
io.open(p, "w", encoding="utf-8", newline="").write(s)
print("fixed")
