import {openNativeImageStorageV1,retireNativeImageStorageV1,retireNativeImageStorageWithWriterV1} from "../../🍎️image/🟦️.ts";
import {capturePersistentNativeImageCleanupV1,capturePersistentNativeProtectedMaintenanceV1,capturePersistentNativeProtectedParentV1,requirePersistentNativeImageCloseV1,retirePersistentNativeFundedChildV1,withPersistentNativeChildStorageV1,withPersistentNativeDisposableChildStorageV1,withPersistentNativeFundedChildStorageV1,withPersistentNativeImageCleanupReadV1,withPersistentNativeMaintenanceChildStorageV1} from "../../🧹️cleanup/🎛️command/🟦️.ts";
import {bindNativeStorageOperationsV1,type NativeStorageOperationsV1} from "../📋️contract/🟦️.ts";
import "./📐️geometry/🟦️.ts";

/** 🍎️ Routes the storage operations to the kernel-mounted image of macOS; the only adapter that reaches the image tooling. */
export const mountedImageStorageOperationsV1:NativeStorageOperationsV1=Object.freeze({
 open:openNativeImageStorageV1,
 retire:retireNativeImageStorageV1,
 retireWithWriter:retireNativeImageStorageWithWriterV1,
 captureCleanup:capturePersistentNativeImageCleanupV1,
 withCleanupRead:withPersistentNativeImageCleanupReadV1,
 requireClose:requirePersistentNativeImageCloseV1,
 withChild:withPersistentNativeChildStorageV1,
 withMaintenanceChild:withPersistentNativeMaintenanceChildStorageV1,
 withDisposableChild:withPersistentNativeDisposableChildStorageV1,
 withFundedChild:withPersistentNativeFundedChildStorageV1,
 retireFundedChild:retirePersistentNativeFundedChildV1,
 captureProtectedParent:capturePersistentNativeProtectedParentV1,
 captureProtectedMaintenance:capturePersistentNativeProtectedMaintenanceV1
});
bindNativeStorageOperationsV1("mounted-image",mountedImageStorageOperationsV1);
