#!/usr/bin/env python3
"""🕸️ Handcrafts the NETWORK-FLOW catalogue entries and merges them into 🖼️assets/🔣️viz-catalog.json.

Owned sections: 8, 9, the network/tree and matrix kinds of 13, 57, 58, 63, 64, 66, 67, and the
network layouts of 75/78. Every kind names one of the families registered by this namespace and a
distinct option set; taxonomy leaves that are synonyms of one another are merged into a single kind
through `covers`, as the architecture asks.

Run:  python 🔧️network-flow-catalog.py
"""

import io
import json
import os
import sys

PRINT = os.path.join(
    'C:\\', 'git', 'semio',
    '\U0001f9f0\ufe0fframework', '\U0001f6cd\ufe0fproducts', '\U0001f4d3\ufe0fprint')
CATALOG = os.path.join(PRINT, '\U0001f5bc\ufe0fassets', '\U0001f523\ufe0fviz-catalog.json')

# region 🔖️Kinds
# Each row: slug, en title, de title, namespace, family, options, demo table, covered leaves.
# The first covered leaf becomes the entry id.
K = [
    # --- 8 general node-link -------------------------------------------------
    ("undirected-graph", "Undirected graph", "Ungerichteter Graph", "network/graph", "graph",
     {"layout": "force", "directed": False, "colorBy": "none"}, "demo-graph-community",
     ["8/undirected-graph"]),
    ("directed-graph", "Directed graph", "Gerichteter Graph", "network/graph", "graph",
     {"layout": "force", "directed": True}, "demo-graph-community",
     ["8/directed-graph", "8/cyclic-graph"]),
    ("weighted-graph", "Weighted graph", "Gewichteter Graph", "network/graph", "graph",
     {"layout": "force", "weightWidth": True}, "demo-graph-community", ["8/weighted-graph"]),
    ("signed-graph", "Signed graph", "Vorzeichenbehafteter Graph", "network/graph", "graph",
     {"layout": "force", "signed": True}, "demo-graph-signed", ["8/signed-graph"]),
    ("multigraph", "Multigraph", "Multigraph", "network/graph", "graph",
     {"layout": "circular", "curvature": 0.18}, "demo-graph-signed",
     ["8/multigraph", "8/pseudograph"]),
    ("bipartite-graph", "Bipartite graph", "Bipartiter Graph", "network/graph", "graph",
     {"layout": "partite", "shape": "square"}, "demo-graph-bipartite", ["8/bipartite-graph"]),
    ("multipartite-graph", "Multipartite graph", "Multipartiter Graph", "network/graph", "graph",
     {"layout": "partite", "shape": "circle"}, "demo-graph-community", ["8/multipartite-graph"]),
    ("dag", "Directed acyclic graph", "Gerichteter azyklischer Graph", "network/graph", "graph",
     {"layout": "layered", "directed": True}, "demo-graph-dag", ["8/dag"]),

    # --- 8 layouts -----------------------------------------------------------
    ("force-directed-graph", "Force-directed graph", "Kräftebasierter Graph",
     "network/graph", "graph",
     {"layout": "force", "iterations": 60, "charge": -30, "linkDistance": 30},
     "demo-graph-community", ["8/force-directed-graph", "78/force"]),
    ("spring-layout", "Spring layout", "Federlayout", "network/graph", "graph",
     {"layout": "force", "iterations": 100, "charge": -45, "linkDistance": 22},
     "demo-graph-community", ["8/spring-layout"]),
    ("fruchterman-reingold-graph", "Fruchterman–Reingold graph", "Fruchterman-Reingold-Graph",
     "network/graph", "graph",
     {"layout": "force", "iterations": 160, "charge": -70, "linkDistance": 26},
     "demo-graph-community", ["8/fruchterman-reingold-graph"]),
    ("kamada-kawai-graph", "Kamada–Kawai graph", "Kamada-Kawai-Graph", "network/graph", "graph",
     {"layout": "force", "iterations": 160, "charge": -20, "linkDistance": 45,
      "forces": "link, many-body, center, collide"},
     "demo-graph-community", ["8/kamada-kawai-graph"]),
    ("circular-graph", "Circular graph", "Kreisgraph", "network/graph", "graph",
     {"layout": "circular", "order": "index"}, "demo-graph-community", ["8/circular-graph"]),
    ("shell-graph", "Shell graph", "Schalengraph", "network/graph", "graph",
     {"layout": "shell", "order": "degree"}, "demo-graph-community", ["8/shell-graph"]),
    ("grid-graph", "Grid graph", "Gittergraph", "network/graph", "graph",
     {"layout": "grid", "order": "name"}, "demo-graph-community", ["8/grid-graph"]),
    ("spectral-layout", "Spectral layout", "Spektrallayout", "network/graph", "graph",
     {"layout": "spectral", "sizeBy": "degree"}, "demo-graph-community", ["8/spectral-layout"]),
    ("random-layout", "Random layout", "Zufallslayout", "network/graph", "graph",
     {"layout": "random", "seed": 1}, "demo-graph-community", ["8/random-layout"]),
    ("hierarchical-graph", "Hierarchical graph", "Hierarchischer Graph", "network/graph", "graph",
     {"layout": "layered", "orientation": "vertical", "directed": True}, "demo-graph-dag",
     ["8/hierarchical-graph"]),
    ("layered-sugiyama-graph", "Layered (Sugiyama) graph", "Geschichteter Sugiyama-Graph",
     "network/graph", "graph",
     {"layout": "layered", "orientation": "horizontal", "directed": True, "sweeps": 6,
      "sizeBy": "degree"},
     "demo-graph-dag", ["8/layered-sugiyama-graph", "75/dag-layout"]),
    ("radial-network", "Radial network", "Radiales Netz", "network/graph", "graph",
     {"layout": "radial", "root": 1}, "demo-graph-community", ["8/radial-network"]),

    # --- 8 ego and community -------------------------------------------------
    ("ego-network", "Ego network", "Ego-Netzwerk", "network/graph", "graph",
     {"layout": "radial", "root": 1, "sizeBy": "degree", "labels": True},
     "demo-graph-community", ["8/ego-network"]),
    ("community-graph", "Community graph", "Gemeinschaftsgraph", "network/graph", "graph",
     {"layout": "force", "iterations": 90, "colorBy": "component"}, "demo-graph-community",
     ["8/community-graph", "8/clustered-network"]),
    ("social-network", "Social network", "Soziales Netzwerk", "network/graph", "graph",
     {"layout": "force", "iterations": 80, "sizeBy": "degree", "labels": True},
     "demo-graph-community", ["8/social-network", "8/collaboration-network"]),
    ("citation-network", "Citation network", "Zitationsnetzwerk", "network/graph", "graph",
     {"layout": "layered", "directed": True, "sizeBy": "degree", "labels": True},
     "demo-graph-dag", ["8/citation-network"]),
    ("knowledge-graph", "Knowledge graph", "Wissensgraph", "network/graph", "graph",
     {"layout": "force", "iterations": 70, "directed": True, "labels": True},
     "demo-graph-community", ["8/knowledge-graph", "8/semantic-network", "13/semantic-network"]),
    ("co-occurrence-network", "Co-occurrence network", "Kookkurrenznetzwerk",
     "network/graph", "graph",
     {"layout": "circular", "order": "degree", "weightWidth": True}, "demo-graph-community",
     ["8/co-occurrence-network", "13/co-occurrence-network"]),
    ("topic-map", "Topic map", "Themenkarte", "network/graph", "graph",
     {"layout": "force", "iterations": 110, "colorBy": "component", "sizeBy": "degree"},
     "demo-graph-community", ["13/topic-map"]),

    # --- 8 dependency family -------------------------------------------------
    ("dependency-graph", "Dependency graph", "Abhängigkeitsgraph", "network/graph", "graph",
     {"layout": "layered", "directed": True, "labels": True}, "demo-graph-dag",
     ["8/dependency-graph", "63/dependency-dag", "64/repository-dependency-graph"]),
    ("call-graph", "Call graph", "Aufrufgraph", "network/graph", "graph",
     {"layout": "layered", "orientation": "vertical", "directed": True, "sizeBy": "degree"},
     "demo-graph-dag", ["8/call-graph"]),
    ("control-flow-graph", "Control-flow graph", "Kontrollflussgraph", "network/graph", "graph",
     {"layout": "layered", "orientation": "vertical", "directed": True, "shape": "box"},
     "demo-graph-dag", ["8/control-flow-graph"]),
    ("data-flow-graph", "Data-flow graph", "Datenflussgraph", "network/graph", "graph",
     {"layout": "layered", "directed": True, "weightWidth": True, "shape": "diamond"},
     "demo-graph-dag", ["8/data-flow-graph", "58/data-flow-threat-diagram"]),
    ("package-dependency-graph", "Package dependency graph", "Paketabhängigkeitsgraph",
     "network/graph", "graph",
     {"layout": "force", "iterations": 80, "directed": True, "colorBy": "component"},
     "demo-graph-community", ["8/package-dependency-graph", "8/module-graph"]),
    ("build-graph", "Build graph", "Build-Graph", "network/graph", "graph",
     {"layout": "layered", "directed": True, "sweeps": 8, "shape": "box"}, "demo-graph-dag",
     ["8/build-graph"]),
    ("graph-neural-network", "Graph neural network", "Graph-neuronales Netz",
     "network/graph", "graph",
     {"layout": "force", "iterations": 80, "colorBy": "group", "sizeBy": "degree"},
     "demo-graph-community", ["67/graph-neural-network"]),

    # --- 8 dense alternatives ------------------------------------------------
    ("adjacency-matrix", "Adjacency matrix", "Adjazenzmatrix", "network/matrix",
     "adjacency-matrix", {"order": "index", "scaleBy": "weight"}, "demo-graph-community",
     ["8/adjacency-matrix"]),
    ("connection-matrix", "Connection matrix", "Verbindungsmatrix", "network/matrix",
     "adjacency-matrix", {"order": "name", "scaleBy": "binary", "diagonal": True},
     "demo-graph-community", ["8/connection-matrix"]),
    ("clustered-adjacency-matrix", "Clustered adjacency matrix", "Gruppierte Adjazenzmatrix",
     "network/matrix", "adjacency-matrix", {"order": "cluster", "blocks": True},
     "demo-graph-community", ["58/mitre-style-technique-matrix"]),
    ("access-control-matrix", "Access-control matrix", "Zugriffsmatrix", "network/matrix",
     "adjacency-matrix",
     {"order": "name", "scaleBy": "binary", "symmetric": False, "diagonal": True},
     "demo-graph-bipartite", ["58/access-control-matrix"]),
    ("attention-heatmap", "Attention heatmap", "Attention-Heatmap", "network/matrix",
     "adjacency-matrix",
     {"order": "index", "scaleBy": "weight", "symmetric": False, "diagonal": True},
     "demo-graph-community", ["67/attention-heatmap"]),
    ("document-term-matrix", "Document–term matrix", "Dokument-Term-Matrix", "network/matrix",
     "adjacency-matrix", {"order": "group", "scaleBy": "weight", "symmetric": False},
     "demo-graph-bipartite", ["13/document-term-matrix"]),
    ("sentiment-matrix", "Sentiment matrix", "Sentiment-Matrix", "network/matrix",
     "adjacency-matrix", {"order": "name", "scaleBy": "weight", "symmetric": False,
                          "diagonal": True}, "demo-graph-signed", ["13/sentiment-matrix"]),
    ("contribution-matrix", "Contribution matrix", "Beitragsmatrix", "network/matrix",
     "adjacency-matrix", {"order": "degree", "scaleBy": "weight", "symmetric": False},
     "demo-graph-community", ["64/contribution-heatmap"]),
    ("node-link-matrix-hybrid", "Node-link matrix hybrid", "Knoten-Kanten-Matrix-Hybrid",
     "network/matrix", "node-link-matrix", {"order": "degree"}, "demo-graph-community",
     ["8/node-link-matrix-hybrid"]),
    ("biofabric-style-graph", "BioFabric-style graph", "BioFabric-Graph", "network/matrix",
     "biofabric", {"order": "group"}, "demo-graph-community", ["8/biofabric-style-graph"]),

    # --- 8 arcs and hives ----------------------------------------------------
    ("arc-diagram", "Arc diagram", "Bogendiagramm", "network/arc", "arc-diagram",
     {"mode": "linear", "side": "above", "order": "index"}, "demo-graph-community",
     ["8/arc-diagram", "75/arc-layout"]),
    ("circular-arc-diagram", "Circular arc diagram", "Kreisbogendiagramm", "network/arc",
     "arc-diagram", {"mode": "circular", "order": "degree"}, "demo-graph-community",
     ["8/circular-arc-diagram"]),
    ("text-arc-diagram", "Text arc diagram", "Text-Bogendiagramm", "network/arc", "arc-diagram",
     {"mode": "linear", "side": "both", "order": "name"}, "demo-graph-community",
     ["13/text-arc-diagram"]),
    ("hive-plot", "Hive plot", "Hive-Plot", "network/arc", "hive-plot",
     {"order": "degree", "innerRadius": 0.3}, "demo-graph-community", ["8/hive-plot"]),

    # --- 8 edge-centric ------------------------------------------------------
    ("chord-diagram", "Chord diagram", "Sehnendiagramm", "network/chord", "chord",
     {"padAngle": 0.04, "symmetric": True}, "demo-graph-community",
     ["8/chord-diagram", "75/chord-layout"]),
    ("dependency-wheel", "Dependency wheel", "Abhängigkeitsrad", "network/chord",
     "dependency-wheel", {"padAngle": 0.03, "symmetric": False, "sortGroups": "descending"},
     "demo-graph-community", ["8/dependency-wheel"]),
    ("edge-bundled-graph", "Edge-bundled graph", "Kantengebündelter Graph", "network/bundling",
     "edge-bundled", {"beta": 0.85, "order": "group"}, "demo-graph-community",
     ["8/edge-bundled-graph", "75/edge-bundling"]),
    ("hierarchical-edge-bundling", "Hierarchical edge bundling",
     "Hierarchische Kantenbündelung", "network/bundling", "edge-bundled",
     {"beta": 0.97, "order": "group", "hubRadius": 0.3}, "demo-graph-community",
     ["8/hierarchical-edge-bundling"]),

    # --- 8 and 57 automata ---------------------------------------------------
    ("automaton", "Automaton", "Automat", "network/graph", "state-machine",
     {"mode": "automaton", "layout": "circular"}, "demo-automaton",
     ["8/automaton", "8/finite-state-machine", "57/finite-automaton",
      "57/deterministic-finite-automaton"]),
    ("nondeterministic-automaton", "Nondeterministic automaton",
     "Nichtdeterministischer Automat", "network/graph", "state-machine",
     {"mode": "automaton", "layout": "circular", "curvature": 0.26}, "demo-automaton",
     ["57/nondeterministic-finite-automaton"]),
    ("pushdown-automaton", "Pushdown automaton", "Kellerautomat", "network/graph",
     "state-machine", {"mode": "automaton", "layout": "layered"}, "demo-automaton",
     ["57/pushdown-automaton"]),
    ("turing-machine-diagram", "Turing-machine diagram", "Turingmaschinen-Diagramm",
     "network/graph", "state-machine", {"mode": "automaton", "layout": "grid"}, "demo-automaton",
     ["57/turing-machine-diagram"]),
    ("state-transition-graph", "State-transition graph", "Zustandsübergangsgraph",
     "network/graph", "state-machine",
     {"mode": "automaton", "layout": "circular", "edgeLabels": True, "curvature": 0.1},
     "demo-automaton", ["8/state-transition-graph", "57/state-transition-diagram"]),
    ("railroad-syntax-diagram", "Railroad syntax diagram", "Eisenbahndiagramm",
     "network/graph", "state-machine",
     {"mode": "automaton", "layout": "arc", "edgeLabels": True}, "demo-automaton",
     ["57/railroad-syntax-diagram"]),
    ("markov-chain-diagram", "Markov-chain diagram", "Markow-Ketten-Diagramm", "network/graph",
     "state-machine", {"mode": "markov", "layout": "circular"}, "demo-automaton",
     ["8/markov-chain-diagram"]),
    ("petri-net", "Petri net", "Petri-Netz", "network/graph", "state-machine",
     {"mode": "petri", "layout": "layered"}, "demo-automaton", ["57/petri-net"]),
    ("bayesian-network", "Bayesian network", "Bayessches Netz", "network/graph", "state-machine",
     {"mode": "causal", "layout": "layered", "orientation": "vertical"}, "demo-graph-dag",
     ["8/bayesian-network"]),
    ("factor-graph", "Factor graph", "Faktorgraph", "network/graph", "state-machine",
     {"mode": "factor", "layout": "partite"}, "demo-graph-bipartite", ["8/factor-graph"]),
    ("causal-dag", "Causal DAG", "Kausaler DAG", "network/graph", "state-machine",
     {"mode": "causal", "layout": "layered"}, "demo-graph-dag", ["8/causal-dag"]),
    ("argument-map", "Argument map", "Argumentkarte", "network/graph", "state-machine",
     {"mode": "argument", "layout": "layered", "orientation": "vertical", "edgeLabels": False,
      "shape": "box"},
     "demo-graph-dag", ["8/argument-map"]),
    ("attack-graph", "Attack graph", "Angriffsgraph", "network/graph", "state-machine",
     {"mode": "attack", "layout": "layered", "edgeLabels": False, "shape": "diamond"},
     "demo-graph-dag", ["8/attack-graph", "58/attack-graph"]),
    ("attack-tree", "Attack tree", "Angriffsbaum", "network/graph", "state-machine",
     {"mode": "attack", "layout": "layered", "orientation": "vertical", "edgeLabels": False,
      "shape": "diamond"},
     "demo-graph-dag", ["58/attack-tree", "58/threat-model"]),

    # --- 58 remaining --------------------------------------------------------
    ("network-topology", "Network topology", "Netztopologie", "network/graph", "graph",
     {"layout": "force", "iterations": 70, "shape": "square", "colorBy": "component"},
     "demo-graph-community", ["58/network-topology"]),
    ("trust-boundary-diagram", "Trust-boundary diagram", "Vertrauensgrenzen-Diagramm",
     "network/graph", "graph", {"layout": "partite", "shape": "box", "labels": True},
     "demo-graph-bipartite", ["58/trust-boundary-diagram"]),
    ("kill-chain-diagram", "Kill-chain diagram", "Kill-Chain-Diagramm", "network/graph", "graph",
     {"layout": "arc", "directed": True, "shape": "box", "labels": True}, "demo-graph-dag",
     ["58/kill-chain-diagram"]),
    ("permission-graph", "Permission graph", "Berechtigungsgraph", "network/graph", "graph",
     {"layout": "partite", "directed": True, "labels": True}, "demo-graph-bipartite",
     ["58/permission-graph"]),
    ("incident-timeline", "Incident timeline", "Vorfall-Zeitstrahl", "network/graph", "graph",
     {"layout": "arc", "directed": True, "labels": True, "order": "index"}, "demo-graph-dag",
     ["58/incident-timeline"]),
    ("alignment-visualization", "Alignment visualization", "Alignment-Darstellung",
     "network/graph", "graph",
     {"layout": "partite", "orientation": "horizontal", "labels": True, "weightWidth": True},
     "demo-graph-bipartite", ["13/alignment-visualization", "13/parallel-text-alignment"]),

    # --- 9 flows -------------------------------------------------------------
    ("sankey-diagram", "Sankey diagram", "Sankey-Diagramm", "flow/sankey", "sankey",
     {"nodeAlign": "justify", "nodePadding": 2.4}, "demo-flow",
     ["9/sankey-diagram", "75/sankey-layout", "78/sankey"]),
    ("alluvial-diagram", "Alluvial diagram", "Alluvialdiagramm", "flow/alluvial", "alluvial",
     {"nodePadding": 2.0}, "demo-flow", ["9/alluvial-diagram"]),
    ("parallel-sets", "Parallel sets", "Parallele Mengen", "flow/parallelsets", "parallel-sets",
     {"nodeWidth": 3.0}, "demo-flow-stages", ["9/parallel-sets"]),
    ("energy-flow-diagram", "Energy-flow diagram", "Energieflussdiagramm", "flow/sankey",
     "sankey", {"nodeAlign": "left", "colorBy": "target", "nodePadding": 1.6}, "demo-flow",
     ["9/energy-flow-diagram"]),
    ("material-flow-diagram", "Material-flow diagram", "Stoffflussdiagramm", "flow/sankey",
     "sankey", {"nodeAlign": "center", "nodeWidth": 5.0}, "demo-flow",
     ["9/material-flow-diagram"]),
    ("money-flow-diagram", "Money-flow diagram", "Geldflussdiagramm", "flow/sankey", "sankey",
     {"nodeAlign": "right", "colorBy": "source", "nodeWidth": 6.0}, "demo-flow",
     ["9/money-flow-diagram"]),
    ("migration-flow-diagram", "Migration-flow diagram", "Migrationsflussdiagramm",
     "flow/sankey", "sankey",
     {"nodeAlign": "justify", "nodePadding": 1.2, "colorBy": "target"}, "demo-flow",
     ["9/migration-flow-diagram"]),
    ("traffic-flow-diagram", "Traffic-flow diagram", "Verkehrsflussdiagramm", "flow/sankey",
     "sankey", {"nodeAlign": "left", "nodeWidth": 2.5, "curvature": 0.35}, "demo-flow",
     ["9/traffic-flow-diagram"]),
    ("user-flow-diagram", "User-flow diagram", "Nutzerflussdiagramm", "flow/alluvial",
     "alluvial", {"nodePadding": 2.4, "colorBy": "source"}, "demo-flow-stages",
     ["9/user-flow-diagram"]),
    ("conversion-flow-diagram", "Conversion-flow diagram", "Konversionsflussdiagramm",
     "flow/alluvial", "alluvial", {"nodePadding": 1.6, "colorBy": "target"}, "demo-flow-stages",
     ["9/conversion-flow-diagram"]),
    ("funnel-flow-diagram", "Funnel-flow diagram", "Trichterflussdiagramm", "flow/sankey",
     "sankey", {"nodeAlign": "left", "nodeSort": "none"}, "demo-flow-stages",
     ["9/funnel-flow-diagram"]),
    ("river-flow-diagram", "River-flow diagram", "Flussdiagramm", "flow/alluvial", "alluvial",
     {"nodePadding": 0.6, "linkSort": "none"}, "demo-flow-stages",
     ["9/river-flow-diagram", "9/themeriver"]),
    ("flow-network", "Flow network", "Flussnetzwerk", "flow/sankey", "sankey",
     {"nodeAlign": "justify", "linkSort": "none", "iterations": 10}, "demo-flow",
     ["9/flow-network"]),
    ("source-sink-graph", "Source–sink graph", "Quelle-Senke-Graph", "flow/sankey", "sankey",
     {"nodeAlign": "left", "nodeSort": "none", "iterations": 1}, "demo-flow",
     ["9/source-sink-graph"]),
    ("input-output-flow-diagram", "Input–output flow diagram", "Input-Output-Flussdiagramm",
     "flow/sankey", "sankey", {"nodeAlign": "center", "iterations": 12}, "demo-flow",
     ["9/input-output-flow-diagram"]),

    # --- 57 data structures --------------------------------------------------
    ("trie", "Trie", "Trie", "network/graph", "data-structure", {"mode": "trie"},
     "demo-structure", ["57/trie"]),
    ("suffix-tree", "Suffix tree", "Suffixbaum", "network/graph", "data-structure",
     {"mode": "suffix-tree"}, "demo-structure", ["57/suffix-tree"]),
    ("heap-visualization", "Heap", "Heap", "network/graph", "data-structure", {"mode": "heap"},
     "demo-structure", ["57/heap-visualization"]),
    ("binary-search-tree", "Binary search tree", "Binärer Suchbaum", "network/graph",
     "data-structure", {"mode": "bst"}, "demo-structure", ["57/binary-search-tree"]),
    ("red-black-tree", "Red–black tree", "Rot-Schwarz-Baum", "network/graph", "data-structure",
     {"mode": "red-black"}, "demo-structure", ["57/red-black-tree"]),
    ("b-tree", "B-tree", "B-Baum", "network/graph", "data-structure", {"mode": "b-tree"},
     "demo-structure", ["57/b-tree", "66/b-tree-diagram", "66/index-structure"]),
    ("hash-table-diagram", "Hash table", "Hashtabelle", "network/graph", "data-structure",
     {"mode": "hash"}, "demo-structure", ["57/hash-table-diagram"]),
    ("linked-list-diagram", "Linked list", "Verkettete Liste", "network/graph", "data-structure",
     {"mode": "linked-list"}, "demo-structure", ["57/linked-list-diagram"]),
    ("memory-layout-diagram", "Memory layout", "Speicherbelegung", "network/graph",
     "data-structure", {"mode": "memory"}, "demo-structure",
     ["57/memory-layout-diagram", "64/blame-visualization"]),
    ("stack-diagram", "Stack", "Stapel", "network/graph", "data-structure", {"mode": "stack"},
     "demo-structure", ["57/stack-diagram"]),
    ("queue-diagram", "Queue", "Warteschlange", "network/graph", "data-structure",
     {"mode": "queue"}, "demo-structure", ["57/queue-diagram"]),
    ("algorithm-execution-trace", "Algorithm execution trace", "Ablaufprotokoll",
     "network/graph", "data-structure", {"mode": "trace"}, "demo-structure",
     ["57/algorithm-execution-trace"]),
    ("syntax-tree", "Syntax tree", "Syntaxbaum", "network/graph", "data-structure",
     {"mode": "parse-tree"}, "demo-structure",
     ["13/syntax-tree", "13/dependency-parse-tree", "13/constituency-parse-tree"]),
    ("word-tree", "Word tree", "Wortbaum", "network/graph", "data-structure", {"mode": "tree"},
     "demo-structure", ["13/word-tree", "13/phrase-tree"]),
    ("sentence-diagram", "Sentence diagram", "Satzdiagramm", "network/graph", "graph",
     {"layout": "layered", "orientation": "vertical", "shape": "box", "labels": True},
     "demo-graph-dag", ["13/sentence-diagram"]),

    # --- 63 and 64 lineage ---------------------------------------------------
    ("commit-graph", "Commit graph", "Commit-Graph", "network/graph", "commit-graph",
     {"orientation": "horizontal"}, "demo-commits",
     ["64/commit-graph", "63/git-commit-graph", "63/version-history-graph"]),
    ("branch-graph", "Branch graph", "Branch-Graph", "network/graph", "commit-graph",
     {"orientation": "vertical"}, "demo-commits", ["64/branch-graph"]),
    ("merge-graph", "Merge graph", "Merge-Graph", "network/graph", "commit-graph",
     {"orientation": "horizontal", "colorBy": "component"}, "demo-commits", ["64/merge-graph"]),
    ("commit-timeline", "Commit timeline", "Commit-Zeitstrahl", "network/graph", "graph",
     {"layout": "arc", "directed": True, "labels": True, "order": "name"}, "demo-commits",
     ["64/commit-timeline"]),
    ("data-lineage-graph", "Data-lineage graph", "Datenherkunftsgraph", "network/graph", "graph",
     {"layout": "layered", "directed": True, "shape": "box", "labels": True}, "demo-graph-dag",
     ["63/data-lineage-graph", "66/data-lineage"]),
    ("provenance-graph", "Provenance graph", "Provenienzgraph", "network/graph", "graph",
     {"layout": "layered", "directed": True, "colorBy": "component", "shape": "box"},
     "demo-graph-dag", ["63/provenance-graph", "63/workflow-provenance"]),
    ("transformation-graph", "Transformation graph", "Transformationsgraph", "network/graph",
     "graph", {"layout": "layered", "directed": True, "weightWidth": True, "colorBy": "component"},
     "demo-graph-dag", ["63/transformation-graph"]),
    ("pipeline-graph", "Pipeline graph", "Pipeline-Graph", "network/graph", "graph",
     {"layout": "arc", "directed": True, "shape": "box"}, "demo-graph-dag",
     ["63/pipeline-graph"]),

    # --- 66 database ---------------------------------------------------------
    ("er-diagram", "Entity–relationship diagram", "Entity-Relationship-Diagramm",
     "network/graph", "schema-graph", {"mode": "er"}, "demo-graph-bipartite",
     ["66/er-diagram"]),
    ("schema-diagram", "Schema diagram", "Schemadiagramm", "network/graph", "schema-graph",
     {"mode": "schema"}, "demo-graph-bipartite", ["66/schema-diagram"]),
    ("table-relationship-graph", "Table relationship graph", "Tabellenbeziehungsgraph",
     "network/graph", "schema-graph", {"mode": "table-relationship"}, "demo-graph-bipartite",
     ["66/table-relationship-graph"]),
    ("star-schema", "Star schema", "Sternschema", "network/graph", "schema-graph",
     {"mode": "star"}, "demo-graph-bipartite", ["66/star-schema"]),
    ("snowflake-schema", "Snowflake schema", "Schneeflockenschema", "network/graph",
     "schema-graph", {"mode": "snowflake"}, "demo-graph-community", ["66/snowflake-schema"]),
    ("query-plan", "Query plan", "Abfrageplan", "network/graph", "schema-graph",
     {"mode": "query-plan"}, "demo-graph-dag",
     ["66/query-plan", "66/query-execution-tree"]),
    ("join-diagram", "Join diagram", "Join-Diagramm", "network/graph", "schema-graph",
     {"mode": "join"}, "demo-graph-bipartite", ["66/join-diagram"]),

    # --- 67 neural -----------------------------------------------------------
    ("feed-forward-neural-network", "Feed-forward neural network", "Vorwärtsgerichtetes Netz",
     "network/graph", "neural-network",
     {"mode": "feedforward", "render": "units", "connections": "full"}, "demo-layers",
     ["67/feed-forward-neural-network"]),
    ("cnn-architecture", "CNN architecture", "CNN-Architektur", "network/graph",
     "neural-network", {"mode": "cnn", "render": "blocks", "connections": "adjacent"},
     "demo-layers", ["67/cnn-architecture"]),
    ("rnn-architecture", "RNN architecture", "RNN-Architektur", "network/graph",
     "neural-network", {"mode": "rnn", "render": "units", "connections": "adjacent"},
     "demo-layers", ["67/rnn-architecture"]),
    ("lstm-architecture", "LSTM architecture", "LSTM-Architektur", "network/graph",
     "neural-network",
     {"mode": "lstm", "render": "blocks", "connections": "adjacent", "maxUnits": 4},
     "demo-layers", ["67/lstm-architecture"]),
    ("transformer-architecture", "Transformer architecture", "Transformer-Architektur",
     "network/graph", "neural-network",
     {"mode": "transformer", "render": "blocks", "connections": "adjacent", "maxUnits": 8},
     "demo-layers", ["67/transformer-architecture"]),
    ("attention-block", "Attention block", "Attention-Block", "network/graph", "neural-network",
     {"mode": "transformer", "render": "blocks", "connections": "full", "maxUnits": 3},
     "demo-layers", ["67/attention-block"]),
    ("encoder-decoder-diagram", "Encoder–decoder diagram", "Encoder-Decoder-Diagramm",
     "network/graph", "neural-network",
     {"mode": "encoder-decoder", "render": "blocks", "connections": "adjacent", "unitSize": 1.6},
     "demo-layers", ["67/encoder-decoder-diagram"]),
    ("residual-network", "Residual network", "Residuales Netz", "network/graph",
     "neural-network", {"mode": "residual", "render": "units", "connections": "residual"},
     "demo-layers", ["67/residual-network"]),
    ("computational-graph", "Computational graph", "Berechnungsgraph", "network/graph",
     "neural-network",
     {"mode": "computational-graph", "render": "units", "connections": "full", "maxUnits": 5},
     "demo-layers", ["67/computational-graph"]),
    ("tensor-shape-diagram", "Tensor-shape diagram", "Tensorform-Diagramm", "network/graph",
     "neural-network", {"mode": "tensor", "render": "blocks", "connections": "none"},
     "demo-layers", ["67/tensor-shape-diagram"]),
    ("model-pipeline", "Model pipeline", "Modell-Pipeline", "network/graph", "neural-network",
     {"mode": "pipeline", "render": "blocks", "connections": "adjacent", "unitSize": 1.4},
     "demo-layers", ["67/model-pipeline"]),
    ("training-loop", "Training loop", "Trainingsschleife", "network/graph", "neural-network",
     {"mode": "training-loop", "render": "blocks", "connections": "residual"}, "demo-layers",
     ["67/training-loop"]),
    ("diffusion-process-diagram", "Diffusion process diagram", "Diffusionsprozess-Diagramm",
     "network/graph", "neural-network",
     {"mode": "diffusion", "render": "blocks", "connections": "adjacent", "maxUnits": 12},
     "demo-layers", ["67/diffusion-process-diagram"]),
    ("gan-architecture", "GAN architecture", "GAN-Architektur", "network/graph",
     "neural-network",
     {"mode": "gan", "render": "blocks", "connections": "adjacent", "maxUnits": 5},
     "demo-layers", ["67/gan-architecture"]),
    ("autoencoder", "Autoencoder", "Autoencoder", "network/graph", "neural-network",
     {"mode": "autoencoder", "render": "units", "connections": "full", "maxUnits": 8},
     "demo-layers", ["67/autoencoder"]),
]
# endregion 🔖️Kinds

# region 🔖️NotMine
# 🚧 Leaves inside the sections above whose shape is a chart, not a network: they stay with whatever
# the catalogue generator assigned and are listed in 📓️status-NETWORK-FLOW.md as a request.
NOT_MINE = [
    "13/word-cloud", "13/tag-cloud", "13/frequency-cloud", "13/concordance-plot",
    "13/kwic-visualization", "13/text-heatmap", "13/topic-distribution-chart", "13/topic-river",
    "13/topic-evolution-chart", "13/sentiment-timeline", "13/lexical-dispersion-plot",
    "13/vocabulary-growth-curve",
    "64/churn-chart", "64/code-frequency-chart",
    "67/activation-map", "67/feature-map", "67/saliency-map",
]
# endregion 🔖️NotMine


def entries():
    out = []
    for slug, en, de, ns, family, options, data, covers in K:
        opts = {"variant": slug}
        opts.update(options)
        out.append({
            "id": covers[0],
            "slug": slug,
            "title": {"en": en, "de": de},
            "kind": "layout" if covers[0].split("/")[0] in {"75", "78"} else "chart",
            "namespace": ns,
            "family": family,
            "options": opts,
            "data": data,
            "covers": covers,
        })
    return out


def main():
    new = entries()
    slugs = [e["slug"] for e in new]
    assert len(slugs) == len(set(slugs)), "duplicate slug"
    owned = set()
    for e in new:
        for c in e["covers"]:
            assert c not in owned, "leaf covered twice: " + c
            owned.add(c)

    # 🔍 distinctness: no two kinds of one family may share an option set beyond `variant`
    seen = {}
    for e in new:
        key = (e["family"], e["data"],
               tuple(sorted((k, v) for k, v in e["options"].items() if k != "variant")))
        assert key not in seen, "same family and options: " + e["slug"] + " vs " + seen[key]
        seen[key] = e["slug"]

    doc = json.load(io.open(CATALOG, encoding="utf-8"))
    kept = []
    replaced = 0
    for k in doc["kinds"]:
        leaves = set(k["covers"]) | {k["id"]}
        if leaves & owned:
            replaced += 1
            continue
        kept.append(k)
    kept.extend(new)
    kept.sort(key=lambda k: (int(k["id"].split("/")[0]), k["id"].split("/", 1)[1]))
    doc["kinds"] = kept
    payload = (json.dumps(doc, ensure_ascii=False, indent=2) + "\n").encode("utf-8")
    tmp = CATALOG + ".tmp"
    with open(tmp, "wb") as fh:
        fh.write(payload)
    os.replace(tmp, CATALOG)

    out = io.TextIOWrapper(sys.stdout.buffer, encoding="utf-8")
    print("kinds authored: %d covering %d taxonomy leaves" % (len(new), len(owned)), file=out)
    print("catalogue entries replaced: %d, total now %d" % (replaced, len(kept)), file=out)
    print("left to other owners: %d" % len(NOT_MINE), file=out)


if __name__ == "__main__":
    main()
