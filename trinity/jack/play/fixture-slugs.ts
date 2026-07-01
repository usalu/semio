export const TRINITY_JACK_PLAY_FIXTURE_DEFAULT_ID = "nakagin";

const FILE_ID_ALIASES: Record<string, string> = {
  nakagin: "nakagin-capsule-tower",
  "branch-chain": "branch-chain",
};

export function resolveTrinityJackPlayFixtureSlug(slug: string): string | undefined {
  return FILE_ID_ALIASES[slug] ?? slug;
}

export const TRINITY_JACK_PLAY_PRESET_QUERIES: Record<string, string> = {
  nakagin: "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a.name, b.name",
  "branch-chain": "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
};
