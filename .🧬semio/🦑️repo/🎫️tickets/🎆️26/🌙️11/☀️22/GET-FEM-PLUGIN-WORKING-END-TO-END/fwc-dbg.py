import sys
p=r"C:\semio\🧰️framework\🔨️modules\🌱️value\♻️retirement\🎟️tickets\🦀️.rs"
s=open(p,encoding="utf-8",newline="").read()
a='return ticket.native_header_release().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"terminal ticket has no explicit native header lifecycle"))?.demand(ticket.as_ref())}'
b='if ticket.native_header_release().is_none(){eprintln!("[DEBUG] no header {}",std::backtrace::Backtrace::force_capture());}return ticket.native_header_release().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"terminal ticket has no explicit native header lifecycle"))?.demand(ticket.as_ref())}'
mode=sys.argv[1]
if mode=="on":
    assert a in s; s=s.replace(a,b)
else:
    assert b in s; s=s.replace(b,a)
open(p,"w",encoding="utf-8",newline="").write(s)
p2=r"C:\semio\🧰️framework\🛍️products\💻️os\🔨️modules\🔌️plugin\🧪️testing\🧪️artifact-app-laws\🏗️registered-app-construction-and-typed-operation-settlement\🦀️.rs"
s=open(p2,encoding="utf-8",newline="").read()
a='cx.original_turn_ceiling().0);let(mut idle,'
b='{let mut g=cx.original_turn_ceiling().0;g.maximum_copy_bytes*=4;g});let(mut idle,'
if mode=="on":
    assert a in s; s=s.replace(a,b)
else:
    assert b in s; s=s.replace(b,a)
open(p2,"w",encoding="utf-8",newline="").write(s)
