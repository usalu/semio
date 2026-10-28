#include <cairo.h>
#include <pango/pango.h>
#include <uuid/uuid.h>
#include <unicode/uversion.h>
#include <stdio.h>

int main(void) {
  uuid_t id;
  char normalized[37], icu[U_MAX_VERSION_STRING_LENGTH];
  UVersionInfo version;
  if (uuid_parse("AABBCCDD-1122-3344-5566-77889900AABB", id)) return 1;
  uuid_unparse_lower(id, normalized);
  u_getVersion(version);
  u_versionToString(version, icu);
  printf("{\"cairo\":\"%s\",\"pango\":\"%s\",\"icu\":\"%s\",\"uuid\":\"%s\"}\n", cairo_version_string(), pango_version_string(), icu, normalized);
  return 0;
}
