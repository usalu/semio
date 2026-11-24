import { repoCacheDirectory } from "@semio-tech/process/execution";
console.log(repoCacheDirectory(process.argv[2], "tools", "ms-playwright"));
