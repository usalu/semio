from lib import *
NODES=B+"/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs"
def patch_nodes4(t):
    t=once(t,"/** 🪨️ Reports and cancels inference settlement rounds","/** 🎞️ How many times this process entered the owner loop: a read that is repaired, retried or settled again enters it again and its `taken N of M` starts from the records already stored, so the slice report of a worker says how often that happened.\n * @see ../../../🧪️tests/🕸️graph/🩹️repair-rounds */\nlet ownerPassesV1=0;\n\n/** 🪨️ Reports and cancels inference settlement rounds","decl")
    t=once(t,'const samplerOff=startSliceSamplerV1(),reportOff=registerSliceReportSourceV1("owners",()=>({cpuSeconds:','ownerPassesV1++;const samplerOff=startSliceSamplerV1(),reportOff=registerSliceReportSourceV1("owners",()=>({pass:ownerPassesV1,cpuSeconds:',"report")
    return t
TARGETS4={NODES:patch_nodes4}
