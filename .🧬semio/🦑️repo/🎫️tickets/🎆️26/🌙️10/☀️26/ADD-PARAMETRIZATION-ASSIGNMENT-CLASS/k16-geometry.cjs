const fs = require("fs");
const path = require("path");
const root = process.cwd();
const template = path.join(root, "🧰️framework", "🛍️products", "📓️print", "🧾️template", "📝️assignment", "parametrization-geometry.tex");
let source = fs.readFileSync(template, "utf8");
const pairs = [];
const chords = [];
for (let i = 1; i <= 16; i++) for (let j = i + 1; j <= 16; j++) {
  pairs.push([i, j]);
  if (Math.min(j - i, 16 - j + i) >= 6) chords.push([i, j]);
}
let seed = 20261027;
for (let i = chords.length - 1; i > 0; i--) {
  seed = (Math.imul(1664525, seed) + 1013904223) >>> 0;
  const j = seed % (i + 1);
  [chords[i], chords[j]] = [chords[j], chords[i]];
}
const selectedChords = chords.slice(0, 4).sort((a, b) => a[0] - b[0] || a[1] - b[1]);
const selected = Array.from({length: 15}, (_, i) => [i + 1, i + 2]).concat([[1, 16]], selectedChords);
const body = list => list.map(([i, j]) => "  #1{" + i + "}{" + j + "}").join("\n");
source = source.replace(/\\newcommand\{\\AssignmentEdges\}\[1\]\{[\s\S]*?\n\}/, "\\newcommand{\\AssignmentEdges}[1]{\n" + body(pairs) + "\n}");
source = source.replace(/\\newcommand\{\\AssignmentSelectedEdges\}\[1\]\{[\s\S]*?\n\}\n/, "");
source = source.replace("\\newcommand{\\AssignmentMatrixShape}", "\\newcommand{\\AssignmentSelectedEdges}[1]{\n" + body(selected) + "\n}\n\\newcommand{\\AssignmentMatrixShape}");
const graph = String.raw`\newcounter{assignmentvertex}
\newdimen\AssignmentGraphRadiusX
\newdimen\AssignmentGraphRadiusY
\AssignmentGraphRadiusY=32mm
\newsavebox{\AssignmentGraphBox}
\newcommand{\AssignmentGraphEdge}[2]{\draw[draw=semio-chrome-foreground!18!semio-chrome-canvas,line width=.35pt] (v#1) -- (v#2);}
\newcommand{\AssignmentSelectedGraphEdge}[2]{\draw[draw=semio-chrome-foreground!85!semio-chrome-canvas,line width=.8pt] (v#1) -- (v#2);}
\newcommand{\AssignmentGraphVertex}[3]{
  \stepcounter{assignmentvertex}
  \pgfmathsetmacro{\AssignmentAngle}{90-22.5*(\value{assignmentvertex}-1)}
  \node[fill=semio-chrome-canvas,inner sep=1mm,label={[inner xsep=0pt]\AssignmentAngle:{\small\theassignmentvertex}}]
    at (v\theassignmentvertex) {\scalebox{.8}{\AssignmentShape{#1}{#2}{#3}}};
}
\newcommand{\AssignmentGraphPicture}{
  \begin{tikzpicture}[x=\AssignmentGraphRadiusX,y=\AssignmentGraphRadiusY]
    \foreach \i in {1,...,16}{\coordinate (v\i) at ({cos(90-22.5*(\i-1))},{sin(90-22.5*(\i-1))});}
    \AssignmentEdges{\AssignmentGraphEdge}
    \AssignmentSelectedEdges{\AssignmentSelectedGraphEdge}
    \setcounter{assignmentvertex}{0}
    \AssignmentTypesOne{\AssignmentGraphVertex}
    \AssignmentTypesTwo{\AssignmentGraphVertex}
  \end{tikzpicture}
}
\newcommand{\AssignmentCompleteGraph}{
  \begin{center}
    \AssignmentGraphRadiusX=\dimexpr\linewidth/2-14mm\relax
    \sbox{\AssignmentGraphBox}{\ignorespaces\AssignmentGraphPicture\unskip}
    \advance\AssignmentGraphRadiusX by \dimexpr(\linewidth-\wd\AssignmentGraphBox)/2\relax
    \sbox{\AssignmentGraphBox}{\ignorespaces\AssignmentGraphPicture\unskip}
    \usebox{\AssignmentGraphBox}
  \end{center}
}
\newcounter{assignmentedge}
\newdimen\AssignmentLedgerColumnWidth
\newcommand{\AssignmentLedgerEntry}[2]{
  \stepcounter{assignmentedge}
  \pgfmathtruncatemacro{\AssignmentColumn}{floor((\value{assignmentedge}-1)/10)}
  \pgfmathtruncatemacro{\AssignmentRow}{mod(\value{assignmentedge}-1,10)}
  \node[anchor=east,font=\small] at ({.14+\AssignmentColumn},{-6.3*\AssignmentRow}) {#1\,--\,#2};
  \draw[draw=semio-chrome-foreground!65!semio-chrome-canvas,line width=.3pt]
    ({.18+\AssignmentColumn},{-6.3*\AssignmentRow-1.5}) -- ({.98+\AssignmentColumn},{-6.3*\AssignmentRow-1.5});
}
\newcommand{\AssignmentSelectedLedger}{
  \par\noindent
  \begingroup
  \AssignmentLedgerColumnWidth=\dimexpr\linewidth/2\relax
  \begin{tikzpicture}[x=\AssignmentLedgerColumnWidth,y=1mm]
    \path[use as bounding box] (0,3) rectangle (2,-60);
    \setcounter{assignmentedge}{0}
    \AssignmentSelectedEdges{\AssignmentLedgerEntry}
  \end{tikzpicture}
  \endgroup\par
}
`;
const start = source.indexOf("\\newcounter{assignmentvertex}");
const end = source.indexOf("\\newcommand{\\AssignmentBox}", start);
if (start < 0 || end < 0) throw new Error("Assignment geometry block missing");
source = source.slice(0, start) + graph + source.slice(end);
const temporary = template + ".k16-" + process.pid;
fs.writeFileSync(temporary, source);
fs.renameSync(temporary, template);
const report = path.join(root, ".🧬semio", "🦑️repo", "🎫️tickets", "🎆️26", "🌙️10", "☀️26", "ADD-PARAMETRIZATION-ASSIGNMENT-CLASS", "k16-geometry.md");
fs.writeFileSync(report, "# K16 Geometry With Twenty Selected Transitions\n\nThe shared geometry enumerates all 120 unordered pairs of sixteen vertices once, including the 64 pairs between Tables A and B. The complete K16 is drawn faintly. A separate shared enumeration highlights twenty transitions: the sixteen edges of the outer ring, followed by four repeatable random chords through the middle: (1,10), (2,12), (4,10), and (5,13). Their cyclic distances are seven, six, six, and eight. The same twenty-edge callback supplies the writing ledger, so no unselected pair requires an answer.\n\nThe authored selection uses Fisher-Yates with an unsigned 32-bit LCG, seed 20261027, multiplier 1664525 and increment 1013904223. Candidate pairs use ascending endpoints and cyclic distance six through eight. The first four shuffled candidates are sorted by endpoints for stable print order. This script retains the exact generation algorithm.\n\nVertices preserve canonical Table A then Table B order, the original image scale of 0.8, the original 32 mm vertical radius, and the original numbered-node styling. Measuring the natural width and adjusting only the horizontal radius fits the artwork and numeric labels to the full available text width. There is no vertical scaling.\n\nThe selected-edge ledger has two columns of ten rows, a 6.3 mm row pitch, and 68 mm writing lines at 170 mm text width. Its fixed 63 mm footprint fits below the graph on page 3. Both compact exercise tables are unchanged.\n\nValidation: source contracts independently reproduce the random chord selection and compare the K16 edge set against D3. The root agent owns final native PDF verification and delivery. Shared-footer Unicode filename regression diagnosis is retained separately.\n");
console.log(JSON.stringify({geometry: template, report, edgeCount: pairs.length, selectedCount: selected.length, chords: selectedChords}));
