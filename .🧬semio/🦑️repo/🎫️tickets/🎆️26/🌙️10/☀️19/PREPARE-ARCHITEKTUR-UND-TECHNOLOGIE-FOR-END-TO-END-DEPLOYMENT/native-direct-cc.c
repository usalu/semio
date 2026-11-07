#include <windows.h>
#include <stdio.h>

int main(void) {
  if (GetCurrentProcessId() == 0) return 1;
  puts("native-direct-cc");
  return 0;
}
