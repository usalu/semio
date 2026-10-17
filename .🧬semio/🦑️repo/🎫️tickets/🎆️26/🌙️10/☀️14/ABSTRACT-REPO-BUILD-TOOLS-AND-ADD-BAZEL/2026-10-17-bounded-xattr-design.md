# Bounded macOS Xattr Observation Design

Read-only local SDK header and existing helper Source review. No actual xattr/protected/native operation or implementation performed. Missing Python os.listxattr/getxattr is an API limitation, not absence of attributes.

## Primary Local ABI Evidence

`/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/sys/xattr.h:65–79` declares ssize_t fgetxattr(int fd,const char* name,void* value,size_t size,u_int32_t position,int options) and ssize_t flistxattr(int fd,char* namebuff,size_t size,int options), plus pathname variants. Header flags explicitly label XATTR_NOFOLLOW=1 as pathname option; XATTR_MAXNAMELEN=127. Do not apply pathname-only flags to FD calls or Linux ABI signatures. sys/errno.h defines ERANGE34,ENOTSUP45,ELOOP62,ENOATTR93. sys/fcntl.h defines O_NOFOLLOW0x100/O_NONBLOCK0x4 and nofollow-any separately. No web lookup needed.

## Recommended Smallest Method

Extend existing ticket-owned Python diagnostic behind its existing function/schema interface using standard ctypes, only darwin, rather than parsing xattr CLI text. Resolve current-process system symbols via ctypes.CDLL(None,use_errno=True), explicitly declare c_ssize_t return and exact c_int/c_char_p/c_void_p/c_size_t/c_uint32 parameter types. Absent symbols/platform are explicit availablefalse, not successful empty attributes. No external package/runtime protocol dependency.

Open exact already-authorized receipt O_RDONLY|O_NOFOLLOW|O_NONBLOCK, verify ordinary single-link file, before/open/after/named seven-field stamps and exact supplied path bound. FD flistxattr/fgetxattr options0 and position0 observe the retained inode; path nofollow is enforced at open. Parent/name identity checks must remain under existing owner custody. Use fixed4096byte name buffer, reject overflow/error, parse exact NULterminated names, reject empty/duplicates/>32names/>127bytes/name. Sort raw name bytes; serialize names as bounded hex/base64 to avoid Unicode decoding collisions. If human name is included, strict UTF8 decode must be separate optional display DATA.

Read each value once into fixed65536byte buffer, aggregate≤262144bytes, hash exact returned slice with hashlib SHA256 and report length/SHA only. Zero return is a legitimate zero-byte value. Do not allocate from untrusted size probes or treat truncated buffers as complete. Read names again into same fixed bound and require identical sorted vector; keep seven-field before/after/named stamps. This detects many races but is no atomic xattr snapshot, so describe observational scope exactly.

ERANGE is bounded refusal, ENOATTR after successful listing is raced/disappeared attribute refusal, denied/IO/identity errors are failures. ENOTSUP may be explicit availablefalse with closed reason/errno, not empty success. One unavailable record must not silently qualify metadata equality. Preserve actual child status/signal and existing owned transport byte/line caps. No XATTR_NOSECURITY, NODEFAULT or SHOWCOMPRESSION override: ordinary system view only, explicitly scoped rather than all-kernel-metadata claim.

CLI `/usr/bin/xattr` is smaller process plumbing but requires separate stable parsing/binary/name framing and multiple child calls. Existing Python helper already handles hashing/stamps/JSON; ctypes FD calls are the narrower exact-byte extension once ABI Source laws pass.

## Source And Generated Tests

Closed language-neutral cases: empty list/zero attributes; one zero-byte value; Unicode/raw-name distinctions; duplicate/missingNUL/oversizename/list overflow; exactly32names/4096list/65536value/262144aggregate boundaries; returnnegative errno; unavailable symbol/platform/ENOTSUP; ENOATTR afterlist; alias/symlink/multilink/wrongFD refusal; namedinode replacement; each sevenfield mutation; second-list drift; arbitrary primary plus FDclosefailure. Independent SQLite checks accepted state/count/bytes; hashlib can be compared with existing Node crypto oracle. ABI extraction should verify explicit signatures/options/position from locally captured header facts.

Only fresh Generated fixtures should later use real setxattr/fsetxattr as separately authorized test setup, then read-only diagnostic; Node/Bun stat/hash and system xattr utility can independently check expected names/values if available. No protected read or Native publication follows from Source tests. Metadata observations remain causal diagnostic DATA and confer no qualification/cleanup authority. Preserve exact publication stamp equality; do not reset or ignorectime.
