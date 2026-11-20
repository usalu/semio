#include <errno.h>
#include <fcntl.h>
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/attr.h>
#include <sys/stat.h>
#include <unistd.h>

static void read_field(void *value, size_t width, const char **at, const char *end) {
    if ((size_t)(end - *at) < width) exit(3);
    memcpy(value, *at, width);
    *at += width;
}

#define GET(field) read_field(&(field), sizeof(field), &at, end)

static int inspect(int directory) {
    struct attrlist attrs = {
        .bitmapcount = ATTR_BIT_MAP_COUNT,
        .commonattr = ATTR_CMN_RETURNED_ATTRS | ATTR_CMN_ERROR | ATTR_CMN_NAME | ATTR_CMN_DEVID | ATTR_CMN_OBJTYPE | ATTR_CMN_FILEID,
        .dirattr = ATTR_DIR_LINKCOUNT | ATTR_DIR_ALLOCSIZE | ATTR_DIR_DATALENGTH,
        .fileattr = ATTR_FILE_LINKCOUNT | ATTR_FILE_ALLOCSIZE | ATTR_FILE_DATALENGTH | ATTR_FILE_DATAALLOCSIZE,
    };
    _Alignas(8) char data[32768];
    unsigned counts[8] = {0}, missing[8] = {0}, changed[8] = {0};
    int calls = 0;
    for (;;) {
        int pages = getattrlistbulk(directory, &attrs, data, sizeof(data), 0);
        calls++;
        if (pages < 0) { perror("getattrlistbulk"); return 2; }
        if (pages == 0) break;
        const char *entry = data;
        for (int row = 0; row < pages; row++) {
            uint32_t length;
            const char *buffer_end = data + sizeof(data);
            if ((size_t)(buffer_end - entry) < sizeof(length)) return 3;
            memcpy(&length, entry, sizeof(length));
            if (length < 4 + sizeof(attribute_set_t) || length > (size_t)(buffer_end - entry)) return 3;
            const char *at = entry + sizeof(length), *end = entry + length;
            attribute_set_t returned;
            GET(returned);
            uint32_t error = 0, kind = 0, nlink = 0;
            dev_t device = 0;
            uint64_t file = 0;
            int64_t alloc = -1, data_alloc = -1, size = -1;
            if (returned.commonattr & ATTR_CMN_ERROR) GET(error);
            if (error || !(returned.commonattr & ATTR_CMN_NAME)) return 4;
            const char *reference = at;
            attrreference_t name;
            GET(name);
            if (name.attr_dataoffset < 0 || (size_t)name.attr_dataoffset > (size_t)(end - reference) || name.attr_length < 1) return 3;
            const char *named = reference + name.attr_dataoffset;
            if (name.attr_length > (size_t)(end - named) || named[name.attr_length-1] != '\0') return 3;
            if (returned.commonattr & ATTR_CMN_DEVID) GET(device);
            if (returned.commonattr & ATTR_CMN_OBJTYPE) GET(kind);
            if (returned.commonattr & ATTR_CMN_FILEID) GET(file);
            if (returned.dirattr & ATTR_DIR_LINKCOUNT) GET(nlink);
            if (returned.dirattr & ATTR_DIR_ALLOCSIZE) GET(alloc);
            if (returned.dirattr & ATTR_DIR_DATALENGTH) GET(size);
            if (returned.fileattr & ATTR_FILE_LINKCOUNT) GET(nlink);
            if (returned.fileattr & ATTR_FILE_ALLOCSIZE) GET(alloc);
            if (returned.fileattr & ATTR_FILE_DATALENGTH) GET(size);
            if (returned.fileattr & ATTR_FILE_DATAALLOCSIZE) GET(data_alloc);
            struct stat direct;
            if (fstatat(directory, named, &direct, AT_SYMLINK_NOFOLLOW)!=0) { perror("fstatat"); return 2; }
            unsigned item = S_ISREG(direct.st_mode) ? 1 : S_ISDIR(direct.st_mode) ? 2 : S_ISLNK(direct.st_mode) ? 5 : 7;
            counts[item]++;
            int unavailable = !(returned.commonattr & ATTR_CMN_DEVID) || !(returned.commonattr & ATTR_CMN_FILEID) ||
                !(returned.commonattr & ATTR_CMN_OBJTYPE) || alloc < 0 || (item == 1 && (nlink == 0 || size < 0));
            int different = (device != direct.st_dev || file != direct.st_ino || kind != item || alloc != direct.st_blocks * 512 ||
                (item == 1 && (nlink != direct.st_nlink || size != direct.st_size)));
            if (unavailable) missing[item]++;
            else if (different) changed[item]++;
            if (unavailable || different || strcmp(named,"sparse") == 0 || strcmp(named,"f0") == 0)
                printf("%s: masks=%08x,%08x,%08x type=%u link=%u alloc=%" PRId64 " dataAlloc=%" PRId64 " dataLength=%" PRId64 " lstat={type=%u,link=%u,alloc=%" PRId64 ",size=%" PRId64 "} devEqual=%d inoEqual=%d\n",
                    named, returned.commonattr, returned.dirattr, returned.fileattr, kind, nlink, alloc, data_alloc, size,
                    item, direct.st_nlink, (int64_t)direct.st_blocks * 512, (int64_t)direct.st_size, device == direct.st_dev, file == direct.st_ino);
            entry = end;
        }
    }
    printf("calls=%d file=%u/%u/%u directory=%u/%u/%u symlink=%u/%u/%u (seen/missing/different)\n",
        calls, counts[1], missing[1], changed[1], counts[2], missing[2], changed[2], counts[5], missing[5], changed[5]);
    return 0;
}

int main(int argc, char **argv) {
    if (argc != 2) return 1;
    int directory = open(argv[1], O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    if (directory < 0) { perror("open"); return 2; }
    int result = inspect(directory);
    if (close(directory) != 0) { perror("close"); return 2; }
    return result;
}
