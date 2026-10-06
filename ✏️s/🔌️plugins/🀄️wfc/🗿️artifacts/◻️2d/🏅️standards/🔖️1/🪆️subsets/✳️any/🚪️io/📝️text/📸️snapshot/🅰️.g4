// 🅰️ derived from 📖️.grammar.semio — kebab rule names rendered in camelCase.
grammar WfcWfc2dSnapshot;
document : header body ;
header : 'schema' ' ' 'wfc.wfc2d.snapshot' NL ;
body : payload NL? ;
payload : OCTET+ ;
NL : '\n' ;
OCTET : . ;
