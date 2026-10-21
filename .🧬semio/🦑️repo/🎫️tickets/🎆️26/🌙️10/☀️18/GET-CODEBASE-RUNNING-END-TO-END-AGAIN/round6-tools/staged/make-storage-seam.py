#!/usr/bin/env python3
"""Merged copy of the storage-seam set: the live platform file gained the `opening` scope and `directory` (a peer's binding check), so the two g2 rows that rewrite the platform image registry carry them."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from mk import *

T = fresh_copy(C + "/storage-seam-tools", "storage-seam")
patch(T + "/apply.py", 'sys.path.insert(0, os.path.abspath(os.path.join(HERE, "..")))', 'sys.path.insert(0, os.path.abspath(os.path.join(HERE, "..", "..", "..")))')
G2 = T + "/groups/g2.py"
OPENING_TYPE = "opening:Readonly<{directory:string;leaseDirectory:string;filesystemBytes:number;maximumPhysicalBytes:number}>;"
patch(G2, "Readonly<{control:ResourceLeaseControl;leaseDirectory:string;root:string;image", "Readonly<{" + OPENING_TYPE + "control:ResourceLeaseControl;directory:string;leaseDirectory:string;root:string;image")
patch(G2, "Readonly<{backend:string;control:ResourceLeaseControl;leaseDirectory:string;root:string;image", "Readonly<{backend:string;" + OPENING_TYPE + "control:ResourceLeaseControl;directory:string;leaseDirectory:string;root:string;image")
OPENING_VALUE = "opening:Object.freeze({directory:options.directory,leaseDirectory:options.leaseDirectory,filesystemBytes:options.filesystemBytes,maximumPhysicalBytes:options.maximumPhysicalBytes}),"
patch(G2, "platformImages.set(image,Object.freeze({control:options.control,leaseDirectory:options.leaseDirectory,", "platformImages.set(image,Object.freeze({" + OPENING_VALUE + "control:options.control,directory:options.directory,leaseDirectory:options.leaseDirectory,")
patch(G2, "platformImages.set(image,Object.freeze({backend,control:options.control,leaseDirectory:options.leaseDirectory,", "platformImages.set(image,Object.freeze({backend," + OPENING_VALUE + "control:options.control,directory:options.directory,leaseDirectory:options.leaseDirectory,")
print("storage-seam merged copy:", T)
