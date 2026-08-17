#include <mach-o/dyld.h>
#include <dlfcn.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

extern void call_rust_sret0(void *output, void *function);
extern void call_rust_sret1(void *output, void *argument, void *function);

__attribute__((constructor)) static void derive_dwg_fixture(void) {
    uint32_t image_index = UINT32_MAX;
    for (uint32_t index = 0; index < _dyld_image_count(); index++) {
        const char *name = _dyld_get_image_name(index);
        if (name && strstr(name, "semio_s_plugin_stdio-4819ebb3272952d7")) {
            image_index = index;
            break;
        }
    }
    if (image_index == UINT32_MAX) {
        fprintf(stderr, "[DEBUG] stdio test image not loaded\n");
        _exit(2);
    }
    intptr_t slide = _dyld_get_image_vmaddr_slide(image_index);
    void *snapshot = calloc(1, 0x5000);
    uintptr_t string[3] = {0, 0, 0};
    uintptr_t pack[3] = {0, 0, 0};
    void *demo = (void *)(slide + 0x1016e8a38ULL);
    Dl_info info = {0};
    dladdr(demo, &info);
    fprintf(stderr, "[DEBUG] image=%s slide=%#lx snapshot=%p demo=%p resolvedImage=%s base=%p\n", _dyld_get_image_name(image_index), slide, snapshot, demo, info.dli_fname, info.dli_fbase);
    fflush(stderr);
    call_rust_sret0(snapshot, demo);
    fprintf(stderr, "[DEBUG] demo returned\n");
    fflush(stderr);
    call_rust_sret1(string, snapshot, (void *)(slide + 0x1048c5490ULL));
    fprintf(stderr, "[DEBUG] string length=%lu pointer=%#lx capacity=%lu\n", string[0], string[1], string[2]);
    fwrite((const void *)string[1], 1, string[0], stderr);
    call_rust_sret1(pack, snapshot, (void *)(slide + 0x10409e07cULL));
    fprintf(stderr, "[DEBUG] pack length=%lu pointer=%#lx capacity=%lu\n[DEBUG] PACKHEX=", pack[0], pack[1], pack[2]);
    for (uintptr_t index = 0; index < pack[0]; index++) {
        fprintf(stderr, "%02x", ((const unsigned char *)pack[1])[index]);
    }
    fprintf(stderr, "\n");
    _exit(0);
}
