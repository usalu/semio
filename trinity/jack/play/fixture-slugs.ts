export const TRINITY_JACK_PLAY_FIXTURE_DEFAULT_ID = "nakagin";

const FILE_ID_ALIASES: Record<string, string> = {
  nakagin: "nakagin-capsule-tower",
  "branch-chain": "branch-chain",
};

export function resolveTrinityJackPlayFixtureSlug(slug: string): string | undefined {
  return FILE_ID_ALIASES[slug] ?? slug;
}

export const TRINITY_JACK_PLAY_DEFAULT_QUERY =
  "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' AND b.name != 'b' RETURN a.name, b.name, b.label";

export const TRINITY_JACK_PLAY_PRESET_QUERIES: Record<string, string> = {
  nakagin: TRINITY_JACK_PLAY_DEFAULT_QUERY,
  "branch-chain": "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b",
};

export const TRINITY_JACK_PLAY_EXAMPLE_QUERIES: ReadonlyArray<{ readonly id: string; readonly label: string; readonly query: string }> = [
  {
    id: "where-or",
    label: "Where Or",
    query: "MATCH (a:Piece) WHERE a.name = 't_f0_b_c0' OR a.name = 't_f0_b_c1' RETURN a.name",
  },
  {
    id: "return-graph",
    label: "Return Graph",
    query: "MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'b' RETURN a, r, b",
  },
  {
    id: "set-label",
    label: "Set Label",
    query: "MATCH (a:Piece) WHERE a.name = 'b' SET a.label = 'demo-label'",
  },
  {
    id: "set-position",
    label: "Set Position",
    query: "MATCH (a:Piece) WHERE a.name = 'jack_orphan' SET a.x = 300, a.y = 120",
  },
  {
    id: "create-node",
    label: "Create Node",
    query: "CREATE (n:Piece)",
  },
  {
    id: "create-edge",
    label: "Create Edge",
    query: "CREATE (x:Piece)-[:Connection]->(y:Piece)",
  },
  {
    id: "delete-leaf",
    label: "Delete Leaf",
    query: "MATCH (n:Piece) WHERE n.name = 'jack_prune' DELETE n",
  },
  {
    id: "merge-edge",
    label: "Merge Edge",
    query: "MERGE (x:Piece)-[:Connection]->(y:Piece)",
  },
];
