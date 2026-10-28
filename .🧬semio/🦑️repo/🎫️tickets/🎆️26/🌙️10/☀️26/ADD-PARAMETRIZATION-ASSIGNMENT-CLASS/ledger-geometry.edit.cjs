const fs = require("fs");
const path = require("path");
const target = "🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/parametrization-geometry.tex";
const source = fs.readFileSync(target, "utf8");
const ledger = String.raw`\newcounter{assignmentedge}
\newdimen\AssignmentLedgerColumnWidth
\newcommand{\AssignmentLedgerRows}{10}
\newcommand{\AssignmentLedgerEntry}[3]{
  \stepcounter{assignmentedge}
  \pgfmathtruncatemacro{\AssignmentColumn}{floor((\value{assignmentedge}-1)/\AssignmentLedgerRows)}
  \pgfmathtruncatemacro{\AssignmentRow}{mod(\value{assignmentedge}-1,\AssignmentLedgerRows)}
  \node[anchor=east,font=\small] at ({.14+\AssignmentColumn},{-6.3*\AssignmentRow}) {#1\,$\to$\,#2};
  \draw[draw=semio-chrome-foreground!65!semio-chrome-canvas,line width=.3pt]
    ({.18+\AssignmentColumn},{-6.3*\AssignmentRow-1.5}) -- ({.94+\AssignmentColumn},{-6.3*\AssignmentRow-1.5});
  \if\relax\detokenize{#3}\relax\else
    \node[anchor=base west,inner sep=0pt,font=\fontsize{9}{10}\selectfont]
      at ({.18+\AssignmentColumn},{-6.3*\AssignmentRow-.7}) {#3};
  \fi
}
\newcommand{\AssignmentBlankLedgerEntry}[2]{
  \AssignmentLedgerEntry{#1}{#2}{}
}
\newcommand{\AssignmentLedger}[2]{
  \par
  \begingroup
  \renewcommand{\AssignmentLedgerRows}{#1}
  \AssignmentLedgerColumnWidth=\dimexpr\linewidth/2\relax
  \noindent\begin{tikzpicture}[x=\AssignmentLedgerColumnWidth,y=1mm]
    \path[use as bounding box] (0,3) rectangle (2,{-6.3*(#1-1)-3.3});
    \setcounter{assignmentedge}{0}
    #2
  \end{tikzpicture}
  \endgroup\par
}
\newcommand{\AssignmentSelectedLedger}{
  \AssignmentLedger{10}{\AssignmentSelectedEdges{\AssignmentBlankLedgerEntry}}
}
\newcommand{\AssignmentWorkedExampleLedger}[6]{
  \AssignmentLedger{3}{
    \AssignmentLedgerEntry{1}{2}{#1}
    \AssignmentLedgerEntry{2}{3}{#3}
    \AssignmentLedgerEntry{3}{1}{#6}
    \AssignmentLedgerEntry{2}{1}{#2}
    \AssignmentLedgerEntry{3}{2}{#4}
    \AssignmentLedgerEntry{1}{3}{#5}
  }
}
`;
const graph = String.raw`\newcommand{\AssignmentWorkedExampleGraphPicture}{
  \begin{tikzpicture}[x=\AssignmentGraphRadiusX,y=\AssignmentGraphRadiusY]
    \AssignmentWorkedExampleGraphVertex{1}
    \AssignmentWorkedExampleGraphVertex{2}
    \AssignmentWorkedExampleGraphVertex{3}
    \begin{pgfonlayer}{assignmentbackground}
      \AssignmentSelectedGraphEdge{1}{2}
      \AssignmentSelectedGraphEdge{2}{1}
      \AssignmentSelectedGraphEdge{2}{3}
      \AssignmentSelectedGraphEdge{3}{2}
      \AssignmentSelectedGraphEdge{1}{3}
      \AssignmentSelectedGraphEdge{3}{1}
    \end{pgfonlayer}
  \end{tikzpicture}
}
\newcommand{\AssignmentWorkedExampleGraph}{
  \begin{center}
    \AssignmentGraphRadiusX=\dimexpr\linewidth/2-14mm\relax
    \AssignmentGraphRadiusY=32mm
    \sbox{\AssignmentGraphBox}{\ignorespaces\AssignmentWorkedExampleGraphPicture\unskip}
    \pgfmathsetlength{\AssignmentGraphRadiusX}{\AssignmentGraphRadiusX+(\linewidth-\wd\AssignmentGraphBox)/1.7320508}
    \sbox{\AssignmentGraphBox}{\ignorespaces\AssignmentWorkedExampleGraphPicture\unskip}
    \usebox{\AssignmentGraphBox}
  \end{center}
}
`;
const ledgerStart = source.indexOf("\\newcounter{assignmentedge}");
const ledgerEnd = source.indexOf("\\newcommand{\\AssignmentBox}");
const graphStart = source.indexOf("\\newcommand{\\AssignmentWorkedExampleGraphEdge}");
if (ledgerStart < 0 || ledgerEnd < ledgerStart || graphStart < ledgerEnd) throw Error("Expected geometry sections missing");
const result = source.slice(0,ledgerStart)+ledger+source.slice(ledgerEnd,graphStart)+graph;
const staged = path.join(__dirname,"🗑️generated","ledger-geometry.new.tex");
fs.mkdirSync(path.dirname(staged),{recursive:true});
fs.writeFileSync(staged,result);
if (fs.readFileSync(target,"utf8")!==source) throw Error("Concurrent geometry edit detected");
fs.renameSync(staged,target);
process.stdout.write("[DEBUG] Shared ledger and caption-free graph authored atomically.\n");
