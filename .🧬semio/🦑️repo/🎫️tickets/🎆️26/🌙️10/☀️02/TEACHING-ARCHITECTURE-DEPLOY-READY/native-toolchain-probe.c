#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <sqlite3.h>

int native_c_probe(void) { char text[32]; return snprintf(text, sizeof(text), "%d", SQLITE_VERSION_NUMBER) > 0 && sizeof(uint64_t) == 8 && EXIT_SUCCESS == 0; }
