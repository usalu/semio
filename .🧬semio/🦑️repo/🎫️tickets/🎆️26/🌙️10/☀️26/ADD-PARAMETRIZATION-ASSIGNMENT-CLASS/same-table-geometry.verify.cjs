const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const fixture = JSON.parse(fs.readFileSync(path.join(__dirname, 'same-table-geometry.case.json'), 'utf8'));
const base = '🧰️framework/🛍️products/📓️print/';
const geometry = fs.readFileSync(base + '🧾️template/📝️assignment/parametrization-geometry.tex', 'utf8');
const style = fs.readFileSync(base + '🖋️latex/semio-assignment.sty', 'utf8');
const header = geometry.split('\\newcommand{\\AssignmentWorkedExampleHeader}[1]{')[1].split('\\newcommand{\\AssignmentWorkedExampleTable}')[0];
const table = geometry.split('\\newcommand{\\AssignmentWorkedExampleTable}[7]{')[1].split('\\newcommand{\\AssignmentWorkedExampleGraphVertex}')[0];
for (const [key, value] of Object.entries({
  'type-columns': fixture.matrix.typeColumns,
  'parameter-rows': fixture.matrix.parameterRows,
  'header-height': fixture.matrix.headerHeight,
  'row-height': fixture.matrix.rowHeight,
  padding: fixture.matrix.padding,
  'label-width': fixture.matrix.labelWidth
})) assert.ok(table.includes(key + '=' + value), key);
assert.ok(header.includes('\\scalebox{.5}{\\AssignmentWorkedExampleShape{#1}}'), 'one common half scale');
assert.ok(!header.includes('\\shortstack') && !header.includes('\\scriptsize'), 'unnumbered header');
assert.equal((table.match(/\\AssignmentParameterMatrixRow/g) || []).length, fixture.matrix.populatedRows);
for (const values of fixture.values) assert.ok(table.includes('{' + values.join(',') + '}'), 'unchanged values');
assert.ok(table.includes('\\AssignmentParameterMatrixBlankRows{4}'), 'four shared blank rows');
assert.ok(style.includes('\\NewExpandableDocumentCommand \\AssignmentParameterMatrixBlankRows { m }'), 'expandable public blank rows');
assert.ok(style.includes('\\AssignmentParameterMatrixBlankRows{\\l_semio_assignment_parameter_rows_int}'), 'default uses shared blank rows');
const helper = style.split('\\cs_new:Npn \\semio_assignment_matrix_blank_row: {')[1]?.split('\\NewExpandableDocumentCommand')[0];
assert.ok(helper?.includes('\\multicolumn{1}{|c|}{\\semio_assignment_matrix_writing_strut:}'), 'shared label cell');
assert.ok(helper.includes('\\semio_assignment_matrix_writing_cell:'), 'shared value cells');
assert.ok(!table.includes('\\phantom'), 'no phantom rows');
process.stdout.write('PASS Worked example shares task table dimensions and reusable blank rows.\n');
