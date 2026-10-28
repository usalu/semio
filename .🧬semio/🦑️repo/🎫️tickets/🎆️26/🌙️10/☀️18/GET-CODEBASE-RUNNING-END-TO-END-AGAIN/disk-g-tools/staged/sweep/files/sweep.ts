import budgetPolicy from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import {CACHE_RUNTIME_V1,cacheAreaBudgetBytesV1,type CacheLedgerV1,type CachePartitionBudgetsV1,type CacheSizeLedgerV1} from "../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";

/** 🚦️ What a command that just looked at the size ledger does about caches over their budget: nothing, wait for the interval, or start the background sweep of `areas` (why: the periodic interval or a recorded excess). */
export type BudgetSweepDecisionV1=Readonly<{decision:"enough"|"throttled"|"launch";reason:"interval"|"over-budget"|"none";areas:readonly string[]}>;

/** 🧺️ The ids the generic budget sweep may collect: every area the ledger declares collectable (an area an owner manages keeps its own collector), then the retired roots and the undeclared entries. */
export function budgetSweepAreasV1(ledger:CacheLedgerV1):readonly string[]{return Object.freeze([...ledger.areas.filter(area=>area.collect!==undefined).map(area=>area.id),"retired","undeclared"]);}

/** 📏️ The sweepable areas whose last recorded size is above their budget; an area never measured or without a byte budget is not over. */
export function overBudgetAreasV1(ledger:CacheLedgerV1,sizes:CacheSizeLedgerV1,partitions:CachePartitionBudgetsV1=budgetPolicy.partitions):readonly string[]{
 return Object.freeze(ledger.areas.filter(area=>{
  if(area.collect===undefined)return false;
  const record=sizes.records[area.id],budget=cacheAreaBudgetBytesV1(ledger,area,partitions);
  return record!==undefined&&budget!==null&&record.usedBytes>budget;
 }).map(area=>area.id));
}

/** 🧮️ Pure rule of the budget sweep: a sweep of everything is due once per `sweepIntervalMs`, one of just the areas recorded over their budget once per `overBudgetIntervalMs`; nothing before. The stamp of the last launch is the only memory. */
export function budgetSweepDecisionV1(input:Readonly<{ledger:CacheLedgerV1;sizes:CacheSizeLedgerV1;nowMs:number;lastLaunchMs:number|null;partitions?:CachePartitionBudgetsV1}>,runtime:Pick<typeof CACHE_RUNTIME_V1,"sweepIntervalMs"|"overBudgetIntervalMs">=CACHE_RUNTIME_V1):BudgetSweepDecisionV1{
 const age=input.lastLaunchMs===null?Infinity:input.nowMs-input.lastLaunchMs,over=overBudgetAreasV1(input.ledger,input.sizes,input.partitions);
 if(age>=runtime.sweepIntervalMs)return Object.freeze({decision:"launch" as const,reason:"interval" as const,areas:budgetSweepAreasV1(input.ledger)});
 if(over.length&&age>=runtime.overBudgetIntervalMs)return Object.freeze({decision:"launch" as const,reason:"over-budget" as const,areas:over});
 return Object.freeze({decision:over.length?"throttled" as const:"enough" as const,reason:"none" as const,areas:Object.freeze([])});
}
