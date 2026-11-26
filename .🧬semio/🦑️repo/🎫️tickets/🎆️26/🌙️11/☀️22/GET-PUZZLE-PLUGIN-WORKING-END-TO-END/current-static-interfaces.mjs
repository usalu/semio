import os from "node:os";
import { syncBuiltinESMExports } from "node:module";

const list = os.networkInterfaces.bind(os);
os.networkInterfaces = () => {
  try {
    return list();
  } catch (failure) {
    if (failure.info?.errno !== 1 || failure.info?.syscall !== "uv_interface_addresses") throw failure;
    return {};
  }
};
syncBuiltinESMExports();
