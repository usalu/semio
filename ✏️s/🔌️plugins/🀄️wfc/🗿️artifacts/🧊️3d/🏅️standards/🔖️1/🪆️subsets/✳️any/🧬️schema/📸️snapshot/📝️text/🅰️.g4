grammar Wfc3dSnapshot;

document : header body ;
header : 'schema' SP 'wfc.wfc3d.snapshot' NL ;
body : payload NL? ;
payload : OCTET+ ;

SP : ' ' ;
NL : '\n' ;
OCTET : . ;
