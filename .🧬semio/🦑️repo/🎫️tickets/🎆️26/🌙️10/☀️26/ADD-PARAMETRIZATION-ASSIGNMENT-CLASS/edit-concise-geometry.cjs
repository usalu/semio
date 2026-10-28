const fs = require("node:fs");
const path = require("node:path");
const root = path.resolve("🧰️framework/🛍️products/📓️print");
const generated = path.join(__dirname, "🗑️generated");
fs.mkdirSync(generated, { recursive: true });
const stylePath = path.join(root, "🖋️latex/semio-assignment.sty");
const geometryPath = path.join(root, "🧾️template/📝️assignment/parametrization-geometry.tex");
const row = String.raw`\cs_new:Npn \semio_assignment_matrix_value:n #1 {
  & \multicolumn{1}{c|}{\semio_assignment_matrix_writing_strut: #1}
}
\NewExpandableDocumentCommand \AssignmentParameterMatrixRow { m m } {
  \multicolumn{1}{|c|}{\semio_assignment_matrix_writing_strut: #1}
  \clist_map_function:nN {#2} \semio_assignment_matrix_value:n
  \tabularnewline \hline
}
`;
const previous = String.raw`      \prg_replicate:nn { \l_semio_assignment_parameter_rows_int } {
        \multicolumn{1}{|c|}{\semio_assignment_matrix_writing_strut:}
        \prg_replicate:nn { \l_semio_assignment_type_columns_int } { & \semio_assignment_matrix_writing_cell: }
        \tabularnewline \hline
      }`;
const replacement = String.raw`      \tl_if_blank:nTF {#4} {
        \prg_replicate:nn { \l_semio_assignment_parameter_rows_int } {
          \multicolumn{1}{|c|}{\semio_assignment_matrix_writing_strut:}
          \prg_replicate:nn { \l_semio_assignment_type_columns_int } { & \semio_assignment_matrix_writing_cell: }
          \tabularnewline \hline
        }
      } {#4}`;
const worked = String.raw`\newcommand{\AssignmentWorkedBoxHeader}[4]{
  \shortstack{\AssignmentBox{#1}{#2}{#3}\\[-.5mm]\scriptsize #4}
}
\newcommand{\AssignmentWorkedBoxTable}[3]{
  \begin{AssignmentParameterMatrix}[type-columns=3,parameter-rows=3,header-height=16mm,row-height=6mm,padding=2mm,label-width=27mm]{Parameter}{
    \AssignmentWorkedBoxHeader{2}{1}{1}{1} &
    \AssignmentWorkedBoxHeader{3}{1}{1}{2} &
    \AssignmentWorkedBoxHeader{3}{2}{1}{3}
  }
    \AssignmentParameterMatrixRow{#1}{2,3,3}
    \AssignmentParameterMatrixRow{#2}{1,1,2}
    \AssignmentParameterMatrixRow{#3}{1,1,1}
  \end{AssignmentParameterMatrix}
}
\newcommand{\AssignmentWorkedBoxGraphVertex}[4]{
  \pgfmathsetmacro{\AssignmentAngle}{90-120*(#1-1)}
  \node[fill=semio-chrome-canvas,inner sep=1mm,label={[inner xsep=0pt]\AssignmentAngle:{\small #1}}]
    at (v#1) {\AssignmentBox{#2}{#3}{#4}};
}
\newcommand{\AssignmentWorkedBoxGraphPicture}{
  \begin{tikzpicture}[x=\AssignmentGraphRadiusX,y=\AssignmentGraphRadiusY]
    \foreach \i in {1,...,3}{\coordinate (v\i) at ({cos(90-120*(\i-1))},{sin(90-120*(\i-1))});}
    \AssignmentSelectedGraphEdge{1}{2}
    \AssignmentSelectedGraphEdge{2}{3}
    \AssignmentSelectedGraphEdge{1}{3}
    \AssignmentWorkedBoxGraphVertex{1}{2}{1}{1}
    \AssignmentWorkedBoxGraphVertex{2}{3}{1}{1}
    \AssignmentWorkedBoxGraphVertex{3}{3}{2}{1}
  \end{tikzpicture}
}
\newcommand{\AssignmentWorkedBoxGraph}{
  \begin{center}
    \AssignmentGraphRadiusX=\dimexpr\linewidth/2-14mm\relax
    \AssignmentGraphRadiusY=16mm
    \sbox{\AssignmentGraphBox}{\ignorespaces\AssignmentWorkedBoxGraphPicture\unskip}
    \pgfmathsetlength{\AssignmentGraphRadiusX}{\AssignmentGraphRadiusX+(\linewidth-\wd\AssignmentGraphBox)/1.7320508}
    \sbox{\AssignmentGraphBox}{\ignorespaces\AssignmentWorkedBoxGraphPicture\unskip}
    \usebox{\AssignmentGraphBox}
  \end{center}
}
\newcommand{\AssignmentWorkedLedgerEntry}[4]{
  \node[anchor=east,font=\small] at ({.18+#1},0) {#2\,--\,#3};
  \node[anchor=west,font=\small] at ({.22+#1},0) {$#4$};
}
\newcommand{\AssignmentWorkedBoxLedger}{
  \par\noindent
  \begingroup
  \AssignmentLedgerColumnWidth=\dimexpr\linewidth/3\relax
  \begin{tikzpicture}[x=\AssignmentLedgerColumnWidth,y=1mm]
    \path[use as bounding box] (0,3) rectangle (3,-3);
    \AssignmentWorkedLedgerEntry{0}{1}{2}{(+1,0,0)}
    \AssignmentWorkedLedgerEntry{1}{2}{3}{(0,+1,0)}
    \AssignmentWorkedLedgerEntry{2}{1}{3}{(+1,+1,0)}
  \end{tikzpicture}
  \endgroup\par
}
`;
let style = fs.readFileSync(stylePath, "utf8");
if (!style.includes(previous)) throw Error("Matrix body mismatch.");
style = style.replace("\\NewDocumentEnvironment { AssignmentParameterMatrix } { O{} m +m }", row + "\\NewDocumentEnvironment { AssignmentParameterMatrix } { O{} m +m +b }").replace(previous, replacement);
let geometry = fs.readFileSync(geometryPath, "utf8");
const index = geometry.indexOf("\\newcommand{\\AssignmentWorkedBoxes}");
if (index < 0) throw Error("Worked boxes mismatch.");
geometry = geometry.slice(0, index) + worked;
for (const [target, content, name] of [[stylePath, style, "concise-style-edited.tmp"], [geometryPath, geometry, "concise-geometry-edited.tmp"]]) {
  const temporary = path.join(generated, name);
  fs.writeFileSync(temporary, content);
  fs.renameSync(temporary, target);
}
