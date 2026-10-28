const fs = require('node:fs');
const changes = [
  [
    "🧾️template/📝️assignment/parametrization-geometry.tex",
    [
      [
        "\\shortstack{\\scalebox{.6}{\\AssignmentWorkedExampleShape{#1}}\\\\[-.5mm]\\scriptsize #1}",
        "\\scalebox{.5}{\\AssignmentWorkedExampleShape{#1}}"
      ],
      [
        "type-columns=3,parameter-rows=4,header-height=20mm",
        "type-columns=3,parameter-rows=8,header-height=16mm"
      ],
      [
        "\\AssignmentParameterMatrixRow{#4}{#5,#6,#7}\n",
        "\\AssignmentParameterMatrixRow{#4}{#5,#6,#7}\n    \\AssignmentParameterMatrixBlankRows{4}\n"
      ]
    ]
  ],
  [
    "🖋️latex/semio-assignment.sty",
    [
      [
        "\\NewDocumentCommand \\AssignmentMatrixImage { m } {",
        "\\cs_new:Npn \\semio_assignment_matrix_blank_row: {\n  \\multicolumn{1}{|c|}{\\semio_assignment_matrix_writing_strut:}\n  \\prg_replicate:nn { \\l_semio_assignment_type_columns_int } { & \\semio_assignment_matrix_writing_cell: }\n  \\tabularnewline \\hline\n}\n\\NewExpandableDocumentCommand \\AssignmentParameterMatrixBlankRows { m } {\n  \\prg_replicate:nn {#1} { \\semio_assignment_matrix_blank_row: }\n}\n\\NewDocumentCommand \\AssignmentMatrixImage { m } {"
      ],
      [
        "        \\prg_replicate:nn { \\l_semio_assignment_parameter_rows_int } {\n          \\multicolumn{1}{|c|}{\\semio_assignment_matrix_writing_strut:}\n          \\prg_replicate:nn { \\l_semio_assignment_type_columns_int } { & \\semio_assignment_matrix_writing_cell: }\n          \\tabularnewline \\hline\n        }",
        "        \\AssignmentParameterMatrixBlankRows{\\l_semio_assignment_parameter_rows_int}"
      ]
    ]
  ]
];
for(const [relative,replacements] of changes){const p='🧰️framework/🛍️products/📓️print/'+relative;let text=fs.readFileSync(p,'utf8');for(const [before,after] of replacements){if(text.split(before).length!==2)throw new Error('Expected one match: '+before);text=text.replace(before,after);}fs.writeFileSync(p+'.new',text);fs.renameSync(p+'.new',p);}
process.stdout.write('Updated shared blank rows and worked table geometry.\n');
