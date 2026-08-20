"use components";
import { nowMs } from './host-shim.js';
import { environment, exit as exit$1, stderr, stdin, stdout, terminalInput, terminalOutput, terminalStderr, terminalStdin, terminalStdout } from '@bytecodealliance/preview2-shim/cli';
import { monotonicClock } from '@bytecodealliance/preview2-shim/clocks';
import { error, poll as poll$2, streams } from '@bytecodealliance/preview2-shim/io';
import { insecureSeed as insecureSeed$1 } from '@bytecodealliance/preview2-shim/random';
const { getEnvironment } = environment;

if (getEnvironment=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getEnvironment', was 'getEnvironment' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { exit } = exit$1;

if (exit=== undefined) {
  const err = new Error("unexpectedly undefined local import 'exit', was 'exit' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getStderr } = stderr;

if (getStderr=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getStderr', was 'getStderr' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getStdin } = stdin;

if (getStdin=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getStdin', was 'getStdin' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getStdout } = stdout;

if (getStdout=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getStdout', was 'getStdout' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { TerminalInput } = terminalInput;

if (TerminalInput=== undefined) {
  const err = new Error("unexpectedly undefined local import 'TerminalInput', was 'TerminalInput' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { TerminalOutput } = terminalOutput;

if (TerminalOutput=== undefined) {
  const err = new Error("unexpectedly undefined local import 'TerminalOutput', was 'TerminalOutput' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getTerminalStderr } = terminalStderr;

if (getTerminalStderr=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getTerminalStderr', was 'getTerminalStderr' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getTerminalStdin } = terminalStdin;

if (getTerminalStdin=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getTerminalStdin', was 'getTerminalStdin' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { getTerminalStdout } = terminalStdout;

if (getTerminalStdout=== undefined) {
  const err = new Error("unexpectedly undefined local import 'getTerminalStdout', was 'getTerminalStdout' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { subscribeDuration } = monotonicClock;

if (subscribeDuration=== undefined) {
  const err = new Error("unexpectedly undefined local import 'subscribeDuration', was 'subscribeDuration' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { Error: Error$1 } = error;

if (Error$1=== undefined) {
  const err = new Error("unexpectedly undefined local import 'Error$1', was 'Error' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { Pollable,
  poll } = poll$2;

if (Pollable=== undefined) {
  const err = new Error("unexpectedly undefined local import 'Pollable', was 'Pollable' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}


if (poll=== undefined) {
  const err = new Error("unexpectedly undefined local import 'poll', was 'poll' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { InputStream,
  OutputStream } = streams;

if (InputStream=== undefined) {
  const err = new Error("unexpectedly undefined local import 'InputStream', was 'InputStream' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}


if (OutputStream=== undefined) {
  const err = new Error("unexpectedly undefined local import 'OutputStream', was 'OutputStream' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}

const { insecureSeed } = insecureSeed$1;

if (insecureSeed=== undefined) {
  const err = new Error("unexpectedly undefined local import 'insecureSeed', was 'insecureSeed' available at instantiation?");
  console.error("ERROR:", err.toString());
  throw err;
}


function promiseWithResolvers() {
  if (Promise.withResolvers) {
    return Promise.withResolvers();
  } else {
    let resolve;
    let reject;
    const promise = new Promise((res, rej) => {
      resolve = res;
      reject = rej;
    });
    return { promise, resolve, reject };
  }
}
const symbolDispose = Symbol.dispose || Symbol.for('dispose');
const symbolAsyncIterator = Symbol.asyncIterator;
const symbolIterator = Symbol.iterator;

const _debugLog = (...args) => {
  if (!globalThis?.process?.env?.JCO_DEBUG) { return; }
  console.debug(...args);
};
const ASYNC_DETERMINISM = 'random';
const GLOBAL_COMPONENT_MEMORY_MAP = new Map();
const CURRENT_TASK_META = {};

function _getGlobalCurrentTaskMeta(componentIdx) {
  if (componentIdx === null || componentIdx === undefined) {
    throw new Error("missing/invalid component idx");
  }
  const v = CURRENT_TASK_META[componentIdx];
  if (v === undefined || v === null) {
    return undefined;
  }
  return { ...v };
}


function _setGlobalCurrentTaskMeta(args) {
  if (!args) { throw new TypeError('args missing'); }
  if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
  if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
  const { taskID, componentIdx } = args;
  return CURRENT_TASK_META[componentIdx] = { taskID, componentIdx };
}


function _withGlobalCurrentTaskMeta(args) {
  _debugLog('[_withGlobalCurrentTaskMeta()] args', args);
  if (!args) { throw new TypeError('args missing'); }
  if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
  if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
  if (!args.fn) { throw new TypeError('missing fn'); }
  const { taskID, componentIdx, fn } = args;
  
  try {
    CURRENT_TASK_META[componentIdx] = { taskID, componentIdx };
    return fn();
  } catch (err) {
    _debugLog("error while executing sync callee/callback", {
      ...args,
      err,
    });
    throw err;
  } finally {
    CURRENT_TASK_META[componentIdx] = null;
  }
}

async function _withGlobalCurrentTaskMetaAsync(args) {
  _debugLog('[_withGlobalCurrentTaskMetaAsync()] args', args);
  if (!args) { throw new TypeError('args missing'); }
  if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
  if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
  if (!args.fn) { throw new TypeError('missing fn'); }
  
  const { taskID, componentIdx, fn } = args;
  
  try {
    CURRENT_TASK_META[componentIdx] = { taskID, componentIdx };
    return await fn();
  } catch (err) {
    _debugLog("error while executing async callee/callback", {
      ...args,
      err,
    });
    throw err;
  } finally {
    CURRENT_TASK_META[componentIdx] = null;
  }
}

async function _clearCurrentTask(args) {
  _debugLog('[_clearCurrentTask()] args', args);
  if (!args) { throw new TypeError('args missing'); }
  if (args.taskID === undefined) { throw new TypeError('missing task ID'); }
  if (args.componentIdx === undefined) { throw new TypeError('missing component idx'); }
  const { taskID, componentIdx } = args;
  
  const meta = CURRENT_TASK_META[componentIdx];
  if (!meta) { throw new Error(`missing current task meta for component idx [${componentIdx}]`); }
  
  if (meta.taskID !== taskID) {
    throw new Error(`task ID [${meta.taskID}] != requested ID [${taskID}]`);
  }
  if (meta.componentIdx !== componentIdx) {
    throw new Error(`component idx [${meta.componentIdx}] != requested idx [${componentIdx}]`);
  }
  
  CURRENT_TASK_META[componentIdx] = null;
}

function lookupMemoriesForComponent(args) {
  const { componentIdx } = args ?? {};
  if (args.componentIdx === undefined) { throw new TypeError("missing component idx"); }
  
  const metas = GLOBAL_COMPONENT_MEMORY_MAP.get(componentIdx);
  if (!metas) { return []; }
  
  if (args.memoryIdx === undefined) {
    return Object.values(metas);
  }
  
  const meta = metas[args.memoryIdx];
  return meta?.memory;
}

function registerGlobalMemoryForComponent(args) {
  const { componentIdx, memory, memoryIdx } = args ?? {};
  if (componentIdx === undefined) { throw new TypeError('missing component idx'); }
  if (memory === undefined && memoryIdx === undefined) { throw new TypeError('missing both memory & memory idx'); }
  let inner = GLOBAL_COMPONENT_MEMORY_MAP.get(componentIdx);
  if (!inner) {
    inner = {};
    GLOBAL_COMPONENT_MEMORY_MAP.set(componentIdx, inner);
  }
  
  inner[memoryIdx] = { memory, memoryIdx, componentIdx };
}

class RepTable {
  #data = [0, null];
  #size = 0;
  #target;
  
  constructor(args) {
    this.target = args?.target;
  }
  
  data() { return this.#data; }
  
  insert(val) {
    _debugLog('[RepTable#insert()] args', { val, target: this.target });
    const freeIdx = this.#data[0];
    if (freeIdx === 0) {
      this.#data.push(val);
      this.#data.push(null);
      const rep = (this.#data.length >> 1) - 1;
      _debugLog('[RepTable#insert()] inserted', { val, target: this.target, rep });
      this.#size += 1;
      return rep;
    }
    this.#data[0] = this.#data[freeIdx << 1];
    const placementIdx = freeIdx << 1;
    this.#data[placementIdx] = val;
    this.#data[placementIdx + 1] = null;
    _debugLog('[RepTable#insert()] inserted', { val, target: this.target, rep: freeIdx });
    this.#size += 1;
    return freeIdx;
  }
  
  get(rep) {
    _debugLog('[RepTable#get()] args', { rep, target: this.target });
    if (rep === 0) { throw new Error('invalid resource rep during get, (cannot be 0)'); }
    
    const baseIdx = rep << 1;
    const val = this.#data[baseIdx];
    return val;
  }
  
  contains(rep) {
    _debugLog('[RepTable#contains()] args', { rep, target: this.target });
    if (rep === 0) { throw new Error('invalid resource rep during contains, (cannot be 0)'); }
    
    const baseIdx = rep << 1;
    return !!this.#data[baseIdx];
  }
  
  remove(rep) {
    _debugLog('[RepTable#remove()] args', { rep, target: this.target });
    if (rep === 0) { throw new Error('invalid resource rep during remove, (cannot be 0)'); }
    if (this.#data.length === 2) { throw new Error('invalid'); }
    
    const baseIdx = rep << 1;
    const val = this.#data[baseIdx];
    
    this.#data[baseIdx] = this.#data[0];
    this.#data[0] = rep;
    this.#size -= 1;
    
    return val;
  }
  
  size() { return this.#size; }
  
  clear() {
    _debugLog('[RepTable#clear()] args', { rep, target: this.target });
    this.#data = [0, null];
  }
}
const _coinFlip = () => { return Math.random() > 0.5; };
let SCOPE_ID = 0;
const I32_MIN = -2_147_483_648;

const I32_MAX= 2_147_483_647;


function _isValidNumericPrimitive(ty, v) {
  if (v === undefined || v === null) { return false; }
  switch (ty) {
    case 'bool':
    return v === 0 || v === 1;
    break;
    case 'u8':
    return v >= 0 && v <= 255;
    break;
    case 's8':
    return v >= -128 && v <= 127;
    break;
    case 'u16':
    return v >= 0 && v <= 65535;
    break;
    case 's16':
    return v >= -32768 && v <= 32767;
    case 'u32':
    return v >= 0 && v <= 4_294_967_295;
    case 's32':
    return v >= -2_147_483_648 && v <= 2_147_483_647;
    case 'u64':
    return typeof v === 'bigint' && v >= 0 && v <= 18_446_744_073_709_551_615n;
    case 's64':
    return typeof v === 'bigint' && v >= -9223372036854775808n && v <= 9223372036854775807n;
    break;
    case 'f32':
    case 'f64': return typeof v === 'number';
    default:
    return false;
  }
  return true;
}

function _requireValidNumericPrimitive(ty, v) {
  if (v === undefined  || v === null || !_isValidNumericPrimitive(ty, v)) {
    throw new TypeError(`invalid ${ty} value [${v}]`);
  }
  return true;
}

const _typeCheckValidI32 = (n) => typeof n === 'number' && n >= I32_MIN && n <= I32_MAX;


const _typeCheckAsyncFn= (f) => {
  return f instanceof ASYNC_FN_CTOR;
};

let RESOURCE_CALL_BORROWS = [];const ASYNC_FN_CTOR = (async () => {}).constructor;

function clearCurrentTask(componentIdx, taskID) {
  _debugLog('[clearCurrentTask()] args', { componentIdx, taskID });
  
  if (componentIdx === undefined || componentIdx === null) {
    throw new Error('missing/invalid component instance index while ending current task');
  }
  
  const tasks = ASYNC_TASKS_BY_COMPONENT_IDX.get(componentIdx);
  if (!tasks || !Array.isArray(tasks)) {
    throw new Error('missing/invalid tasks for component instance while ending task');
  }
  if (tasks.length == 0) {
    throw new Error(`no current tasks for component instance [${componentIdx}] while ending task`);
  }
  
  if (taskID !== undefined) {
    const last = tasks[tasks.length - 1];
    if (last.id !== taskID) {
      // throw new Error('current task does not match expected task ID');
      return;
    }
  }
  
  ASYNC_CURRENT_TASK_IDS.pop();
  ASYNC_CURRENT_COMPONENT_IDXS.pop();
  
  const taskMeta = tasks.pop();
  return taskMeta.task;
}

const CURRENT_TASK_MAY_BLOCK= globalThis.WebAssembly ? new globalThis.WebAssembly.Global({ value: 'i32', mutable: true }, 0) : false;

const ASYNC_CURRENT_TASK_IDS = [];
const ASYNC_CURRENT_COMPONENT_IDXS = [];

function unpackCallbackResult(result) {
  if (!(_typeCheckValidI32(result))) { throw new Error('invalid callback return value [' + result + '], not a valid i32'); }
  const eventCode = result & 0xF;
  if (eventCode < 0 || eventCode > 3) {
    throw new Error('invalid async return value [' + eventCode + '], outside callback code range');
  }
  if (result < 0 || result >= 2**32) { throw new Error('invalid callback result'); }
  // TODO: table max length check?
  const waitableSetRep = result >> 4;
  return [eventCode, waitableSetRep];
}

class AsyncSubtask {
  static _ID = 0n;
  
  static State = {
    STARTING: 0,
    STARTED: 1,
    RETURNED: 2,
    CANCELLED_BEFORE_STARTED: 3,
    CANCELLED_BEFORE_RETURNED: 4,
  };
  
  #id;
  #state = AsyncSubtask.State.STARTING;
  #componentIdx;
  
  #parentTask;
  #childTask = null;
  
  #dropped = false;
  #cancelRequested = false;
  
  #memoryIdx = null;
  #lenders = null;
  
  #waitable = null;
  
  #callbackFn = null;
  #callbackFnName = null;
  
  #postReturnFn = null;
  #onProgressFn = null;
  #pendingEventFn = null;
  
  #callMetadata = {};
  
  #resolved = false;
  
  #onResolveHandlers = [];
  #onStartHandlers = [];
  
  #result = null;
  #resultSet = false;
  
  fnName;
  target;
  isAsync;
  isManualAsync;
  
  constructor(args) {
    if (typeof args.componentIdx !== 'number') {
      throw new Error('invalid componentIdx for subtask creation');
    }
    this.#componentIdx = args.componentIdx;
    
    this.#id = ++AsyncSubtask._ID;
    this.fnName = args.fnName;
    
    if (!args.parentTask) { throw new Error('missing parent task during subtask creation'); }
    this.#parentTask = args.parentTask;
    
    if (args.childTask) { this.#childTask = args.childTask; }
    
    if (args.memoryIdx) { this.#memoryIdx = args.memoryIdx; }
    
    if (!args.waitable) { throw new Error("missing/invalid waitable"); }
    this.#waitable = args.waitable;
    
    if (args.callMetadata) { this.#callMetadata = args.callMetadata; }
    
    this.#lenders = [];
    this.target = args.target;
    this.isAsync = args.isAsync;
    this.isManualAsync = args.isManualAsync;
  }
  
  id() { return this.#id; }
  parentTaskID() { return this.#parentTask?.id(); }
  childTaskID() { return this.#childTask?.id(); }
  state() { return this.#state; }
  
  waitable() { return this.#waitable; }
  waitableRep() { return this.#waitable.idx(); }
  
  join() { return this.#waitable.join(...arguments); }
  getPendingEvent() { return this.#waitable.getPendingEvent(...arguments); }
  hasPendingEvent() { return this.#waitable.hasPendingEvent(...arguments); }
  setPendingEvent() { return this.#waitable.setPendingEvent(...arguments); }
  
  setTarget(tgt) { this.target = tgt; }
  
  getResult() {
    if (!this.#resultSet) { throw new Error("subtask result has not been set") }
    return this.#result;
  }
  setResult(v) {
    if (this.#resultSet) { throw new Error("subtask result has already been set"); }
    this.#result = v;
    this.#resultSet = true;
  }
  
  componentIdx() { return this.#componentIdx; }
  
  setChildTask(t) {
    if (!t) { throw new Error('cannot set missing/invalid child task on subtask'); }
    if (this.#childTask) { throw new Error('child task is already set on subtask'); }
    if (this.#parentTask === t) { throw new Error("parent cannot be child"); }
    this.#childTask = t;
  }
  getChildTask(t) { return this.#childTask; }
  
  getParentTask() { return this.#parentTask; }
  
  setCallbackFn(f, name) {
    if (!f) { return; }
    if (this.#callbackFn) { throw new Error('callback fn can only be set once'); }
    this.#callbackFn = f;
    this.#callbackFnName = name;
  }
  
  getCallbackFnName() {
    if (!this.#callbackFn) { return undefined; }
    return this.#callbackFn.name;
  }
  
  setPostReturnFn(f) {
    if (!f) { return; }
    if (this.#postReturnFn) { throw new Error('postReturn fn can only be set once'); }
    this.#postReturnFn = f;
  }
  
  setOnProgressFn(f) {
    if (this.#onProgressFn) { throw new Error('on progress fn can only be set once'); }
    this.#onProgressFn = f;
  }
  
  isNotStarted() {
    return this.#state == AsyncSubtask.State.STARTING;
  }
  
  registerOnStartHandler(f) {
    this.#onStartHandlers.push(f);
  }
  
  onStart(args) {
    _debugLog('[AsyncSubtask#onStart()] args', {
      componentIdx: this.#componentIdx,
      subtaskID: this.#id,
      parentTaskID: this.parentTaskID(),
      fnName: this.fnName,
      args,
    });
    
    if (this.#onProgressFn) { this.#onProgressFn(); }
    
    this.#state = AsyncSubtask.State.STARTED;
    
    let result;
    
    // If we have been provided a helper start function as a result of
    // component fusion performed by wasmtime tooling, then we can call that helper and lifts/lowers will
    // be performed for us.
    //
    // See also documentation on `HostIntrinsic::PrepareCall`
    //
    if (this.#callMetadata.startFn) {
      result = this.#callMetadata.startFn.apply(null, args?.startFnParams ?? []);
    }
    
    return result;
  }
  
  
  registerOnResolveHandler(f) {
    this.#onResolveHandlers.push(f);
  }
  
  reject(subtaskErr) {
    this.#childTask?.reject(subtaskErr);
  }
  
  onResolve(subtaskValue) {
    _debugLog('[AsyncSubtask#onResolve()] args', {
      componentIdx: this.#componentIdx,
      subtaskID: this.#id,
      isAsync: this.isAsync,
      childTaskID: this.childTaskID(),
      parentTaskID: this.parentTaskID(),
      parentTaskFnName: this.#parentTask?.entryFnName(),
      fnName: this.fnName,
    });
    
    if (this.#resolved) {
      throw new Error('subtask has already been resolved');
    }
    
    if (this.#onProgressFn) { this.#onProgressFn(); }
    
    if (subtaskValue === null && this.#cancelRequested) {
      if (this.#state === AsyncSubtask.State.STARTING) {
        this.#state = AsyncSubtask.State.CANCELLED_BEFORE_STARTED;
      } else {
        if (this.#state !== AsyncSubtask.State.STARTED) {
          throw new Error('resolved subtask must have been started before cancellation');
        }
        this.#state = AsyncSubtask.State.CANCELLED_BEFORE_RETURNED;
      }
    } else {
      if (this.#state !== AsyncSubtask.State.STARTED) {
        throw new Error('resolved subtask must have been started before completion');
      }
      this.#state = AsyncSubtask.State.RETURNED;
    }
    
    this.setResult(subtaskValue);
    
    for (const f of this.#onResolveHandlers) {
      try {
        f(subtaskValue);
      } catch (err) {
        console.error("error during subtask resolve handler", err);
        throw err;
      }
    }
    
    const callMetadata = this.getCallMetadata();
    
    // TODO(fix): we should be able to easily have the caller's meomry
    // to lower into here, but it's not present in PrepareCall
    const memory = callMetadata.memory ?? this.#parentTask?.getReturnMemory() ?? lookupMemoriesForComponent({ componentIdx: this.#parentTask?.componentIdx() })[0];
    if (callMetadata && !callMetadata.returnFn && this.isAsync && callMetadata.resultPtr && memory) {
      const { resultPtr, realloc } = callMetadata;
      const lowers = callMetadata.lowers; // may have been updated in task.return of the child
      if (lowers && lowers.length > 0) {
        lowers[0]({
          componentIdx: this.#componentIdx,
          memory,
          realloc,
          vals: [subtaskValue],
          storagePtr: resultPtr,
          stringEncoding: callMetadata.stringEncoding,
        });
      }
    }
    
    this.#resolved = true;
    this.#parentTask.removeSubtask(this);
    
    if (!this.isAsync) {
      this.deliverResolve();
      const rep = this.waitableRep();
      if (rep) {
        try {
          const removed = this.#getComponentState().handles.remove(rep);
          if (removed !== this) {
            throw new Error("unexpectedly received non-self Subtask from handle removal");
          }
          this.drop();
        } catch (err) {
          _debugLog('[AsyncSubtask#onResolve()] failed to remove subtask after sync subtask completion', err);
        }
      }
    }
  }
  
  getStateNumber() { return this.#state; }
  isReturned() { return this.#state === AsyncSubtask.State.RETURNED; }
  
  getCallMetadata() { return this.#callMetadata; }
  
  isResolved() {
    if (this.#state === AsyncSubtask.State.STARTING
    || this.#state === AsyncSubtask.State.STARTED) {
      return false;
    }
    if (this.#state === AsyncSubtask.State.RETURNED
    || this.#state === AsyncSubtask.State.CANCELLED_BEFORE_STARTED
    || this.#state === AsyncSubtask.State.CANCELLED_BEFORE_RETURNED) {
      return true;
    }
    throw new Error('unrecognized internal Subtask state [' + this.#state + ']');
  }
  
  addLender(handle) {
    _debugLog('[AsyncSubtask#addLender()] args', { handle });
    if (!Number.isNumber(handle)) { throw new Error('missing/invalid lender handle [' + handle + ']'); }
    
    if (this.#lenders.length === 0 || this.isResolved()) {
      throw new Error('subtask has no lendors or has already been resolved');
    }
    
    handle.lends++;
    this.#lenders.push(handle);
  }
  
  deliverResolve() {
    _debugLog('[AsyncSubtask#deliverResolve()] args', {
      lenders: this.#lenders,
      parentTaskID: this.parentTaskID(),
      subtaskID: this.#id,
      childTaskID: this.childTaskID(),
      resolved: this.isResolved(),
      resolveDelivered: this.resolveDelivered(),
    });
    
    const cannotDeliverResolve = this.resolveDelivered() || !this.isResolved();
    if (cannotDeliverResolve) {
      throw new Error('subtask cannot deliver resolution twice, and the subtask must be resolved');
    }
    
    for (const lender of this.#lenders) {
      lender.lends--;
    }
    
    this.#lenders = null;
  }
  
  resolveDelivered() {
    _debugLog('[AsyncSubtask#resolveDelivered()] args', { });
    if (this.#lenders === null && !this.isResolved()) {
      throw new Error('invalid subtask state, lenders missing and subtask has not been resolved');
    }
    return this.#lenders === null;
  }
  
  drop() {
    _debugLog('[AsyncSubtask#drop()] args', {
      componentIdx: this.#componentIdx,
      parentTaskID: this.#parentTask?.id(),
      parentTaskFnName: this.#parentTask?.entryFnName(),
      childTaskID: this.#childTask?.id(),
      childTaskFnName: this.#childTask?.entryFnName(),
      subtaskFnName: this.fnName,
    });
    if (!this.#waitable) { throw new Error('missing/invalid inner waitable'); }
    if (!this.resolveDelivered()) {
      throw new Error('cannot drop subtask before resolve is delivered');
    }
    if (this.#waitable) { this.#waitable.drop() }
    this.#dropped = true;
  }
  
  #getComponentState() {
    const state = getOrCreateAsyncState(this.#componentIdx);
    if (!state) {
      throw new Error('invalid/missing async state for component [' + componentIdx + ']');
    }
    return state;
  }
  
  getWaitableHandleIdx() {
    _debugLog('[AsyncSubtask#getWaitableHandleIdx()] args', { });
    if (!this.#waitable) { throw new Error('missing/invalid waitable'); }
    return this.waitableRep();
  }
}

function _prepareCall(
memoryIdx,
getMemoryFn,
startFn,
returnFn,
callerComponentIdx,
calleeComponentIdx,
taskReturnTypeIdx,
calleeIsAsyncInt,
stringEncoding,
resultCountOrAsync,
) {
  _debugLog('[_prepareCall()]', {
    memoryIdx,
    callerComponentIdx,
    calleeComponentIdx,
    taskReturnTypeIdx,
    calleeIsAsyncInt,
    stringEncoding,
    resultCountOrAsync,
  });
  const argArray = [...arguments];
  
  // value passed in *may* be as large as u32::MAX which may be mangled into -2
  resultCountOrAsync >>>= 0;
  
  let isAsync = false;
  let hasResultPointer = false;
  if (resultCountOrAsync === 2**32 - 1) {
    // prepare async with no result (u32::MAX)
    isAsync = true;
    hasResultPointer = false;
  } else if (resultCountOrAsync === 2**32 - 2) {
    // prepare async with result (u32::MAX - 1)
    isAsync = true;
    hasResultPointer = true;
  }
  
  const currentCallerTaskMeta = getCurrentTask(callerComponentIdx);
  if (!currentCallerTaskMeta) {
    throw new Error('invalid/missing current task for caller during prepare call');
  }
  
  const currentCallerTask = currentCallerTaskMeta.task;
  if (!currentCallerTask) {
    throw new Error('unexpectedly missing task in meta for caller during prepare call');
  }
  
  if (currentCallerTask.componentIdx() !== callerComponentIdx) {
    throw new Error(`task component idx [${ currentCallerTask.componentIdx() }] !== [${ callerComponentIdx }] (callee ${ calleeComponentIdx })`);
  }
  
  let getCalleeParamsFn;
  let resultPtr = null;
  let directParamsArr;
  if (hasResultPointer) {
    directParamsArr = argArray.slice(10, argArray.length - 1);
    getCalleeParamsFn = () => directParamsArr;
    resultPtr = argArray[argArray.length - 1];
  } else {
    directParamsArr = argArray.slice(10);
    getCalleeParamsFn = () => directParamsArr;
  }
  
  let encoding;
  switch (stringEncoding) {
    case 0:
    encoding = 'utf8';
    break;
    case 1:
    encoding = 'utf16';
    break;
    case 2:
    encoding = 'compact-utf16';
    break;
    default:
    throw new Error(`unrecognized string encoding enum [${stringEncoding}]`);
  }
  
  const subtask = currentCallerTask.createSubtask({
    componentIdx: callerComponentIdx,
    parentTask: currentCallerTask,
    isAsync,
    callMetadata: {
      getMemoryFn,
      memoryIdx,
      resultPtr,
      returnFn,
      startFn,
      stringEncoding,
    }
  });
  
  const [newTask, newTaskID] = createNewCurrentTask({
    componentIdx: calleeComponentIdx,
    isAsync,
    getCalleeParamsFn,
    entryFnName: [
    'task',
    subtask.getParentTask().id(),
    'subtask',
    subtask.id(),
    'new-prepared-async-task'
    ].join('/'),
    stringEncoding,
  });
  newTask.setParentSubtask(subtask);
  newTask.setReturnMemoryIdx(memoryIdx);
  newTask.setReturnMemory(getMemoryFn);
  subtask.setChildTask(newTask);
  
  newTask.subtaskMeta = {
    subtask,
    calleeComponentIdx,
    callerComponentIdx,
    getCalleeParamsFn,
    stringEncoding,
    isAsync,
  };
  
  _setGlobalCurrentTaskMeta({
    taskID: newTask.id(),
    componentIdx: newTask.componentIdx(),
  });
}

function _asyncStartCall(args, callee, paramCount, resultCount, flags) {
  const componentIdx = ASYNC_CURRENT_COMPONENT_IDXS.at(-1);
  
  const globalTaskMeta = _getGlobalCurrentTaskMeta(componentIdx);
  if (!globalTaskMeta) { throw new Error('missing global current task globalTaskMeta'); }
  const taskID = globalTaskMeta.taskID;
  
  _debugLog('[_asyncStartCall()] args', { args, componentIdx });
  const { getCallbackFn, callbackIdx, getPostReturnFn, postReturnIdx } = args;
  
  const preparedTaskMeta = getCurrentTask(componentIdx, taskID);
  if (!preparedTaskMeta) { throw new Error('unexpectedly missing current task'); }
  
  const preparedTask = preparedTaskMeta.task;
  if (!preparedTask) { throw new Error('unexpectedly missing current task'); }
  if (!preparedTask.subtaskMeta) { throw new Error('missing subtask meta from prepare'); }
  
  const {
    subtask,
    returnMemoryIdx,
    getReturnMemoryFn,
    callerComponentIdx,
    calleeComponentIdx,
    getCalleeParamsFn,
    isAsync,
    stringEncoding,
  } = preparedTask.subtaskMeta;
  if (!subtask) { throw new Error("missing subtask from cstate during async start call"); }
  if (calleeComponentIdx !== preparedTask.componentIdx()) {
    throw new Error(`meta callee idx [${calleeComponentIdx}] != current task idx [${preparedTask.componentIdx()}] during async start call`);
  }
  if (calleeComponentIdx !== componentIdx) {
    throw new Error("mismatched componentIdx for async start call (does not match prepare)");
  }
  
  const argArray = [...arguments];
  
  if (resultCount < 0 || resultCount > 1) { throw new Error('invalid/unsupported result count'); }
  
  const callbackFnName = 'callback_' + callbackIdx;
  const callbackFn = getCallbackFn();
  preparedTask.setCallbackFn(callbackFn, callbackFnName);
  preparedTask.setPostReturnFn(getPostReturnFn());
  
  if (resultCount < 0 || resultCount > 1) {
    throw new Error(`unsupported result count [${ resultCount }]`);
  }
  
  const params = preparedTask.getCalleeParams();
  if (paramCount !== params.length) {
    throw new Error(`unexpected callee param count [${ params.length }], _asyncStartCall invocation expected [${ paramCount }]`);
  }
  
  const callerComponentState = getOrCreateAsyncState(subtask.componentIdx());
  
  const calleeComponentState = getOrCreateAsyncState(preparedTask.componentIdx());
  const calleeBackpressure = calleeComponentState.hasBackpressure();
  
  // Set up a handler on subtask completion to lower results from the call into the caller's memory region.
  //
  // NOTE: during fused guest->guest calls this handler is triggered, but does not actually perform
  // lowering manually, as fused modules provider helper functions that can
  subtask.registerOnResolveHandler((res) => {
    _debugLog('[_asyncStartCall()] handling subtask result', { res, subtaskID: subtask.id() });
    
    let subtaskCallMeta = subtask.getCallMetadata();
    
    // NOTE: in the case of guest -> guest async calls, there may be no memory/realloc present,
    // as the host will intermediate the value storage/movement between calls.
    //
    // We can simply take the value and lower it as a parameter
    if (subtaskCallMeta.memory || subtaskCallMeta.realloc) {
      throw new Error("call metadata unexpectedly contains memory/realloc for guest->guest call");
    }
    
    const callerTask = subtask.getParentTask();
    const calleeTask = preparedTask;
    const callerMemoryIdx = callerTask.getReturnMemoryIdx();
    const callerComponentIdx = callerTask.componentIdx();
    
    // If a helper function was provided we are likely in a fused guest->guest call,
    // and the result will be delivered (lift/lowered) via helper function
    if (subtaskCallMeta && subtaskCallMeta.returnFn) {
      _debugLog('[_asyncStartCall()] return function present while handling subtask result, returning early (skipping lower)', {
        calleeTaskID: calleeTask.id(),
        calleeComponentIdx,
      });
      
      // TODO: centralize calling of returnFn to *one place* (if possible)
      if (subtaskCallMeta.returnFnCalled) { return; }
      
      const res = subtaskCallMeta.returnFn.apply(null, [subtaskCallMeta.resultPtr]);
      
      _debugLog('[_asyncStartCall()] finished calling return fn', {
        calleeTaskID: calleeTask.id(),
        calleeComponentIdx,
        res,
      });
      
      return;
    }
    
    // If there is no where to lower the results, exit early
    if (!subtaskCallMeta.resultPtr) {
      _debugLog('[_asyncStartCall()] no result ptr during subtask result handling, returning early (skipping lower)');
      return;
    }
    
    let callerMemory;
    if (callerMemoryIdx !== null && callerMemoryIdx !== undefined) {
      callerMemory = lookupMemoriesForComponent({ componentIdx: callerComponentIdx, memoryIdx: callerMemoryIdx });
    } else {
      const callerMemories = lookupMemoriesForComponent({ componentIdx: callerComponentIdx });
      if (callerMemories.length !== 1) { throw new Error(`unsupported amount of caller memories`); }
      callerMemory = callerMemories[0];
    }
    
    if (!callerMemory) {
      _debugLog('[_asyncStartCall()] missing memory', { subtaskID: subtask.id(), res });
      throw new Error(`missing memory for to guest->guest call result (subtask [${subtask.id()}])`);
    }
    
    const lowerFns = calleeTask.getReturnLowerFns();
    if (!lowerFns || lowerFns.length === 0) {
      _debugLog('[_asyncStartCall()] missing result lower metadata for guest->guest call', { subtaskID: subtask.id() });
      throw new Error(`missing result lower metadata for guest->guest call (subtask [${subtask.id()}])`);
    }
    
    if (lowerFns.length !== 1) {
      _debugLog('[_asyncStartCall()] only single result reportetd for guest->guest call', { subtaskID: subtask.id() });
      throw new Error(`only single result supported for guest->guest calls (subtask [${subtask.id()}])`);
    }
    
    _debugLog('[_asyncStartCall()] lowering results', { subtaskID: subtask.id() });
    lowerFns[0]({
      realloc: undefined,
      memory: callerMemory,
      vals: [res],
      storagePtr: subtaskCallMeta.resultPtr,
      componentIdx: callerComponentIdx,
      stringEncoding: subtaskCallMeta.stringEncoding,
    });
    
  });
  
  subtask.setOnProgressFn(() => {
    subtask.setPendingEvent(() => {
      if (subtask.isResolved()) { subtask.deliverResolve(); }
      const event = {
        code: ASYNC_EVENT_CODE.SUBTASK,
        payload0: subtask.waitableRep(),
        payload1: subtask.getStateNumber(),
      };
      return event;
    });
  });
  
  // Start the (event) driver loop that will resolve the subtask
  // in a new JS task
  setTimeout(async () => {
    _debugLog('[_asyncStartCall()] continuing started subtask (in JS task)', {
      taskID: preparedTask.id(),
      subtaskID: subtask.id(),
      callerComponentIdx,
      calleeComponentIdx,
    });
    
    let startRes = subtask.onStart({ startFnParams: params });
    startRes = Array.isArray(startRes) ? startRes : [startRes];
    
    if (calleeComponentState.isExclusivelyLocked()) {
      _debugLog('[_asyncStartCall()] during continuation callee is exclusively locked, suspending...', {
        taskID: preparedTask.id(),
        subtaskID: subtask.id(),
        callerComponentIdx,
        calleeComponentIdx,
      });
      await calleeComponentState.suspendTask({
        task: preparedTask,
        readyFn: () => !calleeComponentState.isExclusivelyLocked(),
      });
    }
    
    const started = await preparedTask.enter();
    if (!started) {
      _debugLog('[_asyncStartCall()] task failed early', {
        taskID: preparedTask.id(),
        subtaskID: subtask.id(),
      });
      throw new Error("task failed to start");
      return;
    }
    
    let callbackResult;
    try {
      let jspiCallee;
      if (callee._cachedPromising) {
        jspiCallee = callee._cachedPromising;
      } else {
        callee._cachedPromising = WebAssembly.promising(callee);
        jspiCallee = callee._cachedPromising;
      }
      
      callbackResult = await _withGlobalCurrentTaskMetaAsync({
        taskID: preparedTask.id(),
        componentIdx: preparedTask.componentIdx(),
        fn: () => {
          return jspiCallee.apply(null, startRes);
        }
      });
    } catch(err) {
      _debugLog("[_asyncStartCall()] initial subtask callee run failed", err);
      // NOTE: a good place to rejectt the parent task, if rejection API is enabled
      // subtask.reject(err);
      // subtask.getParentTask().reject(err);
      
      subtask.getParentTask().setErrored(err);
      
      return;
    }
    
    // If there was no callback function, we're dealing with a sync function
    // that was lifted as async without one, there is only the callee.
    if (!callbackFn) {
      _debugLog("[_asyncStartCall()] no callback, resolving w/ callee result", {
        taskID: preparedTask.id(),
        componentIdx: preparedTask.componentIdx(),
        preparedTask,
        stateNumber: preparedTask.taskState(),
        isResolved: preparedTask.isResolved(),
        callbackFn,
      });
      preparedTask.resolve([callbackResult]);
      return;
    }
    
    let fnName = callbackFn.fnName;
    if (!fnName) {
      fnName = [
      '<task ',
      subtask.parentTaskID(),
      '/subtask ',
      subtask.id(),
      '/task ',
      preparedTask.id(),
      '>',
      ].join("");
    }
    
    try {
      _debugLog("[_asyncStartCall()] starting driver loop", {
        fnName,
        componentIdx: preparedTask.componentIdx(),
        subtaskID: subtask.id(),
        childTaskID: subtask.childTaskID(),
        parentTaskID: subtask.parentTaskID(),
      });
      
      await _driverLoop({
        componentState: calleeComponentState,
        task: preparedTask,
        fnName,
        isAsync: true,
        callbackResult,
        resolve,
        reject
      });
    } catch (err) {
      _debugLog("[AsyncStartCall] drive loop call failure", { err });
    }
    
  }, 0);
  
  const subtaskState = subtask.getStateNumber();
  if (subtaskState < 0 || subtaskState > 2**5) {
    throw new Error('invalid subtask state, out of valid range');
  }
  
  _debugLog('[_asyncStartCall()] returning subtask rep & state', {
    subtask: {
      rep: subtask.waitableRep(),
      state: subtaskState,
    }
  });
  
  return Number(subtask.waitableRep()) << 4 | subtaskState;
}

function _syncStartCall(callbackIdx) {
  _debugLog('[_syncStartCall()] args', { callbackIdx });
  throw new Error('synchronous start call not implemented!');
}

class Waitable {
  #componentIdx;
  
  #pendingEventFn = null;
  
  #promise;
  #resolve;
  #reject;
  
  #waitableSet = null;
  
  #hasSyncWaiter = false;
  
  #idx = null; // to component-global waitables
  
  target;
  
  constructor(args) {
    const { componentIdx, target } = args;
    this.#componentIdx = componentIdx;
    this.target = args.target;
    this.#resetPromise();
  }
  
  componentIdx() { return this.#componentIdx; }
  isInSet() { return this.#waitableSet !== null; }
  
  idx() { return this.#idx; }
  setIdx(idx) {
    if (idx === 0) { throw new Error("waitable idx cannot be zero"); }
    this.#idx = idx;
  }
  
  setTarget(tgt) { this.target = tgt; }
  
  #resetPromise() {
    const { promise, resolve, reject } = promiseWithResolvers()
    this.#promise = promise;
    this.#resolve = resolve;
    this.#reject = reject;
  }
  
  resolve() { this.#resolve(); }
  reject(err) { this.#reject(err); }
  promise() { return this.#promise; }
  
  hasPendingEvent() {
    // _debugLog('[Waitable#hasPendingEvent()]', {
      //     componentIdx: this.#componentIdx,
      //     waitable: this,
      //     waitableSet: this.#waitableSet,
      //     hasPendingEvent: this.#pendingEventFn !== null,
      // });
      return this.#pendingEventFn !== null;
    }
    
    setPendingEvent(fn) {
      _debugLog('[Waitable#setPendingEvent()] args', {
        waitable: this,
        inSet: this.#waitableSet,
      });
      this.#pendingEventFn = fn;
    }
    
    getPendingEvent() {
      _debugLog('[Waitable#getPendingEvent()] args', {
        waitable: this,
        inSet: this.#waitableSet,
        hasPendingEvent: this.#pendingEventFn !== null,
      });
      if (this.#pendingEventFn === null) { return null; }
      const eventFn = this.#pendingEventFn;
      this.#pendingEventFn = null;
      const e = eventFn();
      this.#resetPromise();
      return e;
    }
    
    join(waitableSet) {
      _debugLog('[Waitable#join()] args', {
        waitable: this,
        waitableSet: waitableSet,
        isRemoval: waitableSet === null,
      });
      
      if (this.#waitableSet === undefined) {
        throw new TypeError('waitable set must be not be undefined');
      }
      
      if (this.#waitableSet) {
        this.#waitableSet.removeWaitable(this);
      }
      
      this.#waitableSet = waitableSet;
      
      if (waitableSet) {
        this.#waitableSet.addWaitable(this);
      }
    }
    
    drop() {
      _debugLog('[Waitable#drop()] args', {
        componentIdx: this.#componentIdx,
        waitable: this,
      });
      if (this.hasPendingEvent()) {
        throw new Error('waitables with pending events cannot be dropped');
      }
      this.join(null);
    }
    
    async waitForPendingEvent(args) {
      const { cstate } = args;
      if (!cstate) { throw new TypeError('missing component state'); }
      
      if (this.#waitableSet !== null || this.#hasSyncWaiter) {
        throw new Error("waitable is already in a set/has a sync waiter");
      }
      this.#hasSyncWaiter = true;
      await cstate.waitUntil({
        cancellable: false,
        readyFn: () => this.hasPendingEvent(),
      });
      this.#hasSyncWaiter = false;
    }
    
  }
  
  const ERR_CTX_TABLES = {};
  
  function contextGet(ctx) {
    const { componentIdx, slot } = ctx;
    if (componentIdx === undefined) { throw new TypeError("missing component idx"); }
    if (slot === undefined) { throw new TypeError("missing slot"); }
    
    const currentTaskMeta = _getGlobalCurrentTaskMeta(componentIdx);
    if (!currentTaskMeta) {
      throw new Error(`missing/incomplete global current task meta for component idx [${componentIdx}] during context set`);
    }
    const taskID = currentTaskMeta.taskID;
    
    const taskMeta = getCurrentTask(componentIdx, taskID);
    if (!taskMeta) { throw new Error('failed to retrieve current task'); }
    
    let task = taskMeta.task;
    if (!task) { throw new Error('invalid/missing current task in metadata while getting context'); }
    
    _debugLog('[contextGet()] args', {
      slot,
      storage: task.storage,
      taskID: task.id(),
      componentIdx: task.componentIdx(),
    });
    
    if (slot < 0 || slot >= task.storage.length) { throw new Error('invalid slot for current task'); }
    
    return task.storage[slot];
  }
  
  
  function contextSet(ctx, value) {
    const { componentIdx, slot } = ctx;
    if (componentIdx === undefined) { throw new TypeError("missing component idx"); }
    if (slot === undefined) { throw new TypeError("missing slot"); }
    if (!(_typeCheckValidI32(value))) { throw new Error('invalid value for context set (not valid i32)'); }
    
    const currentTaskMeta = _getGlobalCurrentTaskMeta(componentIdx);
    if (!currentTaskMeta) {
      throw new Error(`missing/incomplete global current task meta for component idx [${componentIdx}] during context set`);
    }
    const taskID = currentTaskMeta.taskID;
    
    const taskMeta = getCurrentTask(componentIdx, taskID);
    if (!taskMeta) { throw new Error('failed to retrieve current task'); }
    
    let task = taskMeta.task;
    if (!task) { throw new Error('invalid/missing current task in metadata while setting context'); }
    
    _debugLog('[contextSet()] args', {
      slot,
      value,
      storage: task.storage,
      taskID: task.id(),
      componentIdx: task.componentIdx(),
    });
    
    if (slot < 0 || slot >= task.storage.length) { throw new Error('invalid slot for current task'); }
    task.storage[slot] = value;
  }
  
  const ASYNC_TASKS_BY_COMPONENT_IDX = new Map();
  
  class AsyncTask {
    static _ID = 0n;
    
    static State = {
      INITIAL: 'initial',
      CANCELLED: 'cancelled',
      CANCEL_PENDING: 'cancel-pending',
      CANCEL_DELIVERED: 'cancel-delivered',
      RESOLVED: 'resolved',
    }
    
    static BlockResult = {
      CANCELLED: 'block.cancelled',
      NOT_CANCELLED: 'block.not-cancelled',
    }
    
    #id;
    #componentIdx;
    #state;
    #isAsync;
    #isManualAsync;
    #preserveFutureResult;
    #entryFnName = null;
    
    #onResolveHandlers = [];
    #completionPromise = null;
    #rejected = false;
    
    #exitPromise = null;
    #onExitHandlers = [];
    
    #memoryIdx = null;
    #memory = null;
    
    #callbackFn = null;
    #callbackFnName = null;
    
    #postReturnFn = null;
    
    #getCalleeParamsFn = null;
    
    #stringEncoding = null;
    
    #parentSubtask = null;
    
    #errHandling;
    
    #backpressurePromise;
    #backpressureWaiters = 0n;
    
    #returnLowerFns = null;
    
    #subtasks = [];
    
    #entered = false;
    #exited = false;
    #errored = null;
    
    cancelled = false;
    cancelRequested = false;
    alwaysTaskReturn = false;
    
    returnCalls =  0;
    storage = [0, 0];
    borrowedHandles = {};
    
    tmpRetI64HighBits = 0|0;
    
    constructor(opts) {
      this.#id = ++AsyncTask._ID;
      
      if (opts?.componentIdx === undefined) {
        throw new TypeError('missing component id during task creation');
      }
      this.#componentIdx = opts.componentIdx;
      
      this.#state = AsyncTask.State.INITIAL;
      this.#isAsync = opts?.isAsync ?? false;
      this.#isManualAsync = opts?.isManualAsync ?? false;
      this.#preserveFutureResult = opts?.preserveFutureResult ?? false;
      this.#entryFnName = opts.entryFnName;
      
      const {
        promise: completionPromise,
        resolve: resolveCompletionPromise,
        reject: rejectCompletionPromise,
      } = promiseWithResolvers();
      this.#completionPromise = completionPromise;
      
      this.#onResolveHandlers.push((results) => {
        if (this.#parentSubtask !== null) { return; }
        if (!this.#isAsync) { return; }
        
        if (this.#errored !== null) {
          rejectCompletionPromise(this.#errored);
          return;
        } else if (this.#rejected) {
          rejectCompletionPromise(results);
          return;
        }
        
        if (this.#preserveFutureResult && results instanceof FutureValue) {
          results.resolveAsValue(resolveCompletionPromise);
        } else {
          resolveCompletionPromise(results);
        }
      });
      
      const {
        promise: exitPromise,
        resolve: resolveExitPromise,
        reject: rejectExitPromise,
      } = promiseWithResolvers();
      this.#exitPromise = exitPromise;
      
      this.#onExitHandlers.push(() => {
        resolveExitPromise();
      });
      
      if (opts.callbackFn) { this.#callbackFn = opts.callbackFn; }
      if (opts.callbackFnName) { this.#callbackFnName = opts.callbackFnName; }
      
      if (opts.getCalleeParamsFn) { this.#getCalleeParamsFn = opts.getCalleeParamsFn; }
      
      if (opts.stringEncoding) { this.#stringEncoding = opts.stringEncoding; }
      
      if (opts.parentSubtask) { this.#parentSubtask = opts.parentSubtask; }
      
      
      if (opts.errHandling) { this.#errHandling = opts.errHandling; }
    }
    
    taskState() { return this.#state; }
    id() { return this.#id; }
    componentIdx() { return this.#componentIdx; }
    entryFnName() { return this.#entryFnName; }
    
    completionPromise() { return this.#completionPromise; }
    exitPromise() { return this.#exitPromise; }
    
    isAsync() { return this.#isAsync; }
    isSync() { return !this.isAsync(); }
    
    getErrHandling() { return this.#errHandling; }
    
    hasCallback() { return this.#callbackFn !== null; }
    
    getReturnMemoryIdx() { return this.#memoryIdx; }
    setReturnMemoryIdx(idx) {
      if (idx === null) { return; }
      this.#memoryIdx = idx;
    }
    
    getReturnMemory() { return this.#memory; }
    setReturnMemory(m) {
      if (m === null) { return; }
      this.#memory = m;
    }
    
    setReturnLowerFns(fns) { this.#returnLowerFns = fns; }
    getReturnLowerFns() { return this.#returnLowerFns; }
    
    setParentSubtask(subtask) {
      if (!subtask || !(subtask instanceof AsyncSubtask)) { return }
      if (this.#parentSubtask) { throw new Error('parent subtask can only be set once'); }
      this.#parentSubtask = subtask;
    }
    
    getParentSubtask() { return this.#parentSubtask; }
    
    // TODO(threads): this is very inefficient, we can pass along a root task,
    // and ideally do not need this once thread support is in place
    getRootTask() {
      let currentSubtask = this.getParentSubtask();
      let task = this;
      while (currentSubtask) {
        task = currentSubtask.getParentTask();
        currentSubtask = task.getParentSubtask();
      }
      return task;
    }
    
    setPostReturnFn(f) {
      if (!f) { return; }
      if (this.#postReturnFn) { throw new Error('postReturn fn can only be set once'); }
      this.#postReturnFn = f;
    }
    
    setCallbackFn(f, name) {
      if (!f) { return; }
      if (this.#callbackFn) { throw new Error('callback fn can only be set once'); }
      this.#callbackFn = f;
      this.#callbackFnName = name;
    }
    
    getCallbackFnName() {
      if (!this.#callbackFnName) { return undefined; }
      return this.#callbackFnName;
    }
    
    async runCallbackFn(...args) {
      if (!this.#callbackFn) { throw new Error('no callback function has been set for task'); }
      return _withGlobalCurrentTaskMetaAsync({
        taskID: this.#id,
        componentIdx: this.#componentIdx,
        fn: () => { return this.#callbackFn.apply(null, args); }
      });
    }
    
    getCalleeParams() {
      if (!this.#getCalleeParamsFn) { throw new Error('missing/invalid getCalleeParamsFn'); }
      return this.#getCalleeParamsFn();
    }
    
    mayBlock() { return this.isAsync() || this.isResolvedState() }
    
    mayEnter(task) {
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      if (cstate.hasBackpressure()) {
        _debugLog('[AsyncTask#mayEnter()] disallowed due to backpressure', { taskID: this.#id });
        return false;
      }
      if (!cstate.callingSyncImport()) {
        _debugLog('[AsyncTask#mayEnter()] disallowed due to sync import call', { taskID: this.#id });
        return false;
      }
      const callingSyncExportWithSyncPending = cstate.callingSyncExport && !task.isAsync;
      if (!callingSyncExportWithSyncPending) {
        _debugLog('[AsyncTask#mayEnter()] disallowed due to sync export w/ sync pending', { taskID: this.#id });
        return false;
      }
      return true;
    }
    
    enterSync() {
      if (this.needsExclusiveLock()) {
        const cstate = getOrCreateAsyncState(this.#componentIdx);
        // TODO(???): it is *very possible* for a the line below to fail if
        // an async function is already running (and holding the exclusive lock)
        //
        // It's not really possible to fix this unless we turn every sync export into
        // an async export that will use the regular async enabled `enter()`.
        cstate.exclusiveLock();
      }
      return true;
    }
    
    async enter(opts) {
      _debugLog('[AsyncTask#enter()] args', {
        taskID: this.#id,
        componentIdx: this.#componentIdx,
        subtaskID: this.getParentSubtask()?.id(),
        args: opts,
        entryFnName: this.#entryFnName,
      });
      
      if (this.#entered) {
        throw new Error(`task with ID [${this.#id}] should not be entered twice`);
      }
      
      const cstate = getOrCreateAsyncState(this.#componentIdx);
      
      if (opts?.isHost) {
        this.#entered = true;
        return this.#entered;
      }
      
      await cstate.nextTaskExecutionSlot({ task: this });
      
      // If a task is synchronous then we can avoid component-relevant
      // tracking and immediately enter.
      if (this.isSync()) {
        this.#entered = true;
        
        // TODO(breaking): remove once manually-specifying async fns is removed
        // It is currently possible for an actually sync export to be specified
        // as async via JSPI
        if (this.#isManualAsync) {
          if (this.needsExclusiveLock()) { cstate.exclusiveLock(); }
        }
        
        return this.#entered;
      }
      
      // Perform intial backpressure check
      if (cstate.hasBackpressure() || this.needsExclusiveLock() && cstate.isExclusivelyLocked()) {
        cstate.addBackpressureWaiter();
        
        const result = await this.waitUntil({
          readyFn: () => {
            return !(cstate.hasBackpressure()
            || this.needsExclusiveLock() && cstate.isExclusivelyLocked());
          },
          cancellable: true,
        });
        
        cstate.removeBackpressureWaiter();
        
        if (result === AsyncTask.BlockResult.CANCELLED) {
          this.cancel();
          return false;
        }
      }
      
      // Lock the component state or keep trying until we can/do
      try {
        if (this.needsExclusiveLock()) { cstate.exclusiveLock(); }
      } catch {
        // Continuously attempt to lock until we can
        while (cstate.hasBackpressure() || this.needsExclusiveLock() && cstate.isExclusivelyLocked()) {
          try {
            if (this.needsExclusiveLock()) { cstate.exclusiveLock(); }
            break;
          } catch(err) {
            cstate.addBackpressureWaiter();
            const result = await this.waitUntil({
              readyFn: () => {
                return !(cstate.hasBackpressure()
                || this.needsExclusiveLock() && cstate.isExclusivelyLocked());
              },
              cancellable: true,
            });
            cstate.removeBackpressureWaiter();
            if (result === AsyncTask.BlockResult.CANCELLED) {
              this.cancel();
              return false;
            }
          }
        }
      }
      
      this.#entered = true;
      return this.#entered;
    }
    
    isRunningState() { return this.#state !== AsyncTask.State.RESOLVED; }
    isResolvedState() { return this.#state === AsyncTask.State.RESOLVED; }
    isResolved() { return this.#state === AsyncTask.State.RESOLVED; }
    
    async waitUntil(opts) {
      const { readyFn, cancellable } = opts;
      _debugLog('[AsyncTask#waitUntil()] args', { taskID: this.#id, args: { cancellable } });
      
      // TODO(fix): check for cancel
      // TODO(fix): determinism
      // TODO(threads): add this thread to waiting list
      
      const keepGoing = await this.suspendUntil({
        readyFn,
        cancellable,
      });
      
      return keepGoing;
    }
    
    async yieldUntil(opts) {
      const { readyFn, cancellable } = opts;
      _debugLog('[AsyncTask#yieldUntil()]', {
        taskID: this.#id,
        args: {
          cancellable,
        },
        componentIdx: this.#componentIdx,
      });
      
      const keepGoing = await this.suspendUntil({ readyFn, cancellable });
      if (keepGoing) {
        return {
          code: ASYNC_EVENT_CODE.NONE,
          payload0: 0,
          payload1: 0,
        };
      }
      
      return {
        code: ASYNC_EVENT_CODE.TASK_CANCELLED,
        payload0: 0,
        payload1: 0,
      };
    }
    
    async suspendUntil(opts) {
      const { cancellable, readyFn } = opts;
      _debugLog('[AsyncTask#suspendUntil()] args', {
        taskID: this.#id,
        args: {
          cancellable,
        },
        componentIdx: this.#componentIdx,
      });
      
      const pendingCancelled = this.deliverPendingCancel({ cancellable });
      if (pendingCancelled) { return false; }
      
      const completed = await this.immediateSuspendUntil({ readyFn, cancellable });
      return completed;
    }
    
    // TODO(threads): equivalent to thread.suspend_until()
    async immediateSuspendUntil(opts) {
      const { cancellable, readyFn } = opts;
      _debugLog('[AsyncTask#immediateSuspendUntil()] args', {
        args: {
          cancellable,
          readyFn,
        },
        taskID: this.#id,
        componentIdx: this.#componentIdx,
      });
      
      const ready = readyFn();
      if (ready && ASYNC_DETERMINISM === 'random') {
        const coinFlip = _coinFlip();
        if (coinFlip) { return true }
      }
      
      const keepGoing = await this.immediateSuspend({ cancellable, readyFn });
      return keepGoing;
    }
    
    async immediateSuspend(opts) { // NOTE: equivalent to thread.suspend()
    // TODO(threads): store readyFn on the thread
    const { cancellable, readyFn } = opts;
    _debugLog('[AsyncTask#immediateSuspend()] args', { cancellable, readyFn });
    
    const pendingCancelled = this.deliverPendingCancel({ cancellable });
    if (pendingCancelled) { return false; }
    
    const cstate = getOrCreateAsyncState(this.#componentIdx);
    const keepGoing = await cstate.suspendTask({ task: this, readyFn });
    return keepGoing;
  }
  
  deliverPendingCancel(opts) {
    const { cancellable } = opts;
    _debugLog('[AsyncTask#deliverPendingCancel()]', {
      args: { cancellable },
      taskID: this.#id,
      componentIdx: this.#componentIdx,
    });
    
    if (cancellable && this.#state === AsyncTask.State.PENDING_CANCEL) {
      this.#state = AsyncTask.State.CANCEL_DELIVERED;
      return true;
    }
    
    return false;
  }
  
  isCancelled() { return this.cancelled }
  
  cancel(args) {
    _debugLog('[AsyncTask#cancel()] args', { });
    if (this.taskState() !== AsyncTask.State.CANCEL_DELIVERED) {
      throw new Error(`(component [${this.#componentIdx}]) task [${this.#id}] invalid task state [${this.taskState()}] for cancellation`);
    }
    if (this.borrowedHandles.length > 0) { throw new Error('task still has borrow handles'); }
    this.cancelled = true;
    this.onResolve(args?.error ?? new Error('task cancelled'));
    this.#state = AsyncTask.State.RESOLVED;
  }
  
  onResolve(taskValue) {
    const handlers = this.#onResolveHandlers;
    this.#onResolveHandlers = [];
    for (const f of handlers) {
      try {
        f(taskValue);
      } catch (err) {
        _debugLog("[AsyncTask#onResolve] error during task resolve handler", err);
        throw err;
      }
    }
    
    if (this.#parentSubtask) {
      const meta = this.#parentSubtask.getCallMetadata();
      // Run the rturn fn if it has not already been called -- this *should* have happened in
      // `task.return`, but some paths do not go through task.return (e.g. async lower of sync fn
      // which goes through prepare + async-start-call)
      if (meta.returnFn && !meta.returnFnCalled) {
        _debugLog('[AsyncTask#onResolve()] running returnFn', {
          componentIdx: this.#componentIdx,
          taskID: this.#id,
          subtaskID: this.#parentSubtask.id(),
        });
        const memory = meta.getMemoryFn();
        meta.returnFn.apply(null, [taskValue, meta.resultPtr]);
        meta.returnFnCalled = true;
      }
    }
    
    if (this.#postReturnFn) {
      _debugLog('[AsyncTask#onResolve()] running post return ', {
        componentIdx: this.#componentIdx,
        taskID: this.#id,
      });
      try {
        this.#postReturnFn(taskValue);
      } catch (err) {
        _debugLog("[AsyncTask#onResolve] error during task resolve handler", err);
        throw err;
      }
    }
    
    if (this.#parentSubtask) {
      this.#parentSubtask.onResolve(taskValue);
    }
  }
  
  registerOnResolveHandler(f) {
    this.#onResolveHandlers.push(f);
  }
  
  isRejected() { return this.#rejected; }
  
  isErrored() { return this.#errored; }
  setErrored(err) { this.#errored = err; }
  
  reject(taskErr) {
    _debugLog('[AsyncTask#reject()] args', {
      componentIdx: this.#componentIdx,
      taskID: this.#id,
      parentSubtask: this.#parentSubtask,
      parentSubtaskID: this.#parentSubtask?.id(),
      entryFnName: this.entryFnName(),
      callbackFnName: this.#callbackFnName,
      errMsg: taskErr.message,
    });
    
    if (this.isResolvedState() || this.#rejected) { return; }
    
    this.#rejected = true;
    this.cancelRequested = true;
    this.#state = AsyncTask.State.PENDING_CANCEL;
    const cancelled = this.deliverPendingCancel({ cancellable: true });
    
    // TODO: do cleanup here to reset the machinery so we can run again?
    
    this.cancel({ error: taskErr });
  }
  
  resolve(results) {
    _debugLog('[AsyncTask#resolve()] args', {
      componentIdx: this.#componentIdx,
      taskID: this.#id,
      entryFnName: this.entryFnName(),
      callbackFnName: this.#callbackFnName,
    });
    
    if (this.#state === AsyncTask.State.RESOLVED) {
      throw new Error(`(component [${this.#componentIdx}]) task [${this.#id}]  is already resolved (did you forget to wait for an import?)`);
    }
    
    if (this.borrowedHandles.length > 0) {
      throw new Error('task still has borrow handles');
    }
    
    this.#state = AsyncTask.State.RESOLVED;
    
    switch (results.length) {
      case 0:
      this.onResolve(undefined);
      break;
      case 1:
      this.onResolve(results[0]);
      break;
      default:
      _debugLog('[AsyncTask#resolve()] unexpected number of results', {
        componentIdx: this.#componentIdx,
        results,
        taskID: this.#id,
        subtaskID: this.#parentSubtask?.id(),
        entryFnName: this.#entryFnName,
        callbackFnName: this.#callbackFnName,
      });
      throw new Error('unexpected number of results');
    }
  }
  
  exit(args) {
    _debugLog('[AsyncTask#exit()]', {
      componentIdx: this.#componentIdx,
      taskID: this.#id,
    });
    
    if (this.#exited)  { throw new Error("task has already exited"); }
    
    if (this.#state !== AsyncTask.State.RESOLVED) {
      throw new Error(`(component [${this.#componentIdx}]) task [${this.#id}] exited without resolution`);
    }
    
    if (this.borrowedHandles > 0) {
      throw new Error('task [${this.#id}] exited without clearing borrowed handles');
    }
    
    const state = getOrCreateAsyncState(this.#componentIdx);
    if (!state) { throw new Error('missing async state for component [' + this.#componentIdx + ']'); }
    
    // Exempt the host from exclusive lock check
    if (this.#componentIdx !== -1 && !args?.skipExclusiveLockCheck) {
      if (this.needsExclusiveLock() && !state.isExclusivelyLocked()) {
        throw new Error(`task [${this.#id}] exit: component [${this.#componentIdx}] should have been exclusively locked`);
      }
    }
    
    state.exclusiveRelease();
    
    for (const f of this.#onExitHandlers) {
      try {
        f();
      } catch (err) {
        console.error("error during task exit handler", err);
        throw err;
      }
    }
    
    this.#exited = true;
    clearCurrentTask(this.#componentIdx, this.id());
  }
  
  needsExclusiveLock() {
    return !this.#isAsync || this.hasCallback();
  }
  
  createSubtask(args) {
    _debugLog('[AsyncTask#createSubtask()] args', args);
    const { componentIdx, childTask, callMetadata, fnName, isAsync, isManualAsync } = args;
    
    const cstate = getOrCreateAsyncState(this.#componentIdx);
    if (!cstate) {
      throw new Error(`invalid/missing async state for component idx [${componentIdx}]`);
    }
    
    const waitable = new Waitable({
      componentIdx: this.#componentIdx,
      target: `subtask (internal ID [${this.#id}])`,
    });
    
    const newSubtask = new AsyncSubtask({
      componentIdx,
      childTask,
      parentTask: this,
      callMetadata,
      isAsync,
      isManualAsync,
      fnName,
      waitable,
    });
    this.#subtasks.push(newSubtask);
    newSubtask.setTarget(`subtask (internal ID [${newSubtask.id()}], waitable [${waitable.idx()}], component [${componentIdx}])`);
    waitable.setIdx(cstate.handles.insert(newSubtask));
    waitable.setTarget(`waitable for subtask (waitable id [${waitable.idx()}], subtask internal ID [${newSubtask.id()}])`);
    return newSubtask;
  }
  
  getLatestSubtask() {
    return this.#subtasks.at(-1);
  }
  
  getSubtaskByWaitableRep(rep) {
    if (rep === undefined) { throw new TypeError('missing rep'); }
    return this.#subtasks.find(s => s.waitableRep() === rep);
  }
  
  currentSubtask() {
    _debugLog('[AsyncTask#currentSubtask()]');
    if (this.#subtasks.length === 0) { return undefined; }
    return this.#subtasks.at(-1);
  }
  
  removeSubtask(subtask) {
    if (this.#subtasks.length === 0) {
      throw new Error('cannot end current subtask: no current subtask');
    }
    this.#subtasks = this.#subtasks.filter(t => t !== subtask);
    return subtask;
  }
}

const ASYNC_EVENT_CODE = {
  NONE: 0,
  SUBTASK: 1,
  STREAM_READ: 2,
  STREAM_WRITE: 3,
  FUTURE_READ: 4,
  FUTURE_WRITE: 5,
  TASK_CANCELLED: 6,
};

function getCurrentTask(componentIdx, taskID) {
  let usedGlobal = false;
  if (componentIdx === undefined || componentIdx === null) {
    throw new Error('missing component idx'); // TODO(fix)
    // componentIdx = ASYNC_CURRENT_COMPONENT_IDXS.at(-1);
    // usedGlobal = true;
  }
  
  const taskMetas = ASYNC_TASKS_BY_COMPONENT_IDX.get(componentIdx);
  if (taskMetas === undefined || taskMetas.length === 0) { return undefined; }
  
  if (taskID) {
    return taskMetas.find(meta => meta.task.id() === taskID);
  }
  
  const taskMeta = taskMetas[taskMetas.length - 1];
  if (!taskMeta || !taskMeta.task) { return undefined; }
  
  return taskMeta;
}

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

function toInt64(val) {
  const converted = BigInt(val)
  
  return BigInt.asIntN(64, converted);
}


function toUint64(val) {
  const converted = BigInt(val)
  
  return BigInt.asUintN(64, converted);
}


function toInt32(val) {
  
  return val >> 0;
}


function toUint32(val) {
  
  return val >>> 0;
}

const utf16Decoder = new TextDecoder('utf-16');

function _utf16AllocateAndEncode(str, realloc, memory) {
  const len = str.length;
  const ptr = realloc(0, 0, 2, len * 2);
  const out = new Uint16Array(memory.buffer, ptr, len);
  let i = 0;
  if (isLE) {
    while (i < len) { out[i] = str.charCodeAt(i++); }
  } else {
    while (i < len) {
      const ch = str.charCodeAt(i);
      out[i++] = (ch & 0xff) << 8 | ch >>> 8;
    }
  }
  return { ptr, len, codepoints: [...str].length };
}

const TEXT_DECODER_UTF8 = new TextDecoder();
const TEXT_ENCODER_UTF8 = new TextEncoder();

function _utf8AllocateAndEncode(s, realloc, memory) {
  if (typeof s !== 'string') {
    throw new TypeError('expected a string, received [' + typeof s + ']');
  }
  if (s.length === 0) { return { ptr: 1, len: 0 }; }
  let buf = TEXT_ENCODER_UTF8.encode(s);
  let ptr = realloc(0, 0, 1, buf.length);
  new Uint8Array(memory.buffer).set(buf, ptr);
  const res = { ptr, len: buf.length, codepoints: [...s].length };
  return res;
}


async function _utf8AllocateAndEncodeAsync(s, realloc, memory) {
  if (typeof s !== 'string') {
    throw new TypeError('expected a string, received [' + typeof s + ']');
  }
  if (s.length === 0) { return { ptr: 1, len: 0 }; }
  let buf = TEXT_ENCODER_UTF8.encode(s);
  let ptr = await realloc(0, 0, 1, buf.length);
  new Uint8Array(memory.buffer).set(buf, ptr);
  const res = { ptr, len: buf.length, codepoints: [...s].length };
  return res;
}


const T_FLAG = 1 << 30;

function rscTableCreateOwn(table, rep) {
  const free = table[0] & ~T_FLAG;
  table._createdReps.add(rep);
  if (free === 0) {
    table.push(0);
    table.push(rep | T_FLAG);
    return (table.length >> 1) - 1;
  }
  table[0] = table[free << 1];
  table[free << 1] = 0;
  table[(free << 1) + 1] = rep | T_FLAG;
  return free;
}

function rscTableRemove(table, handle) {
  const scope = table[handle << 1];
  const val = table[(handle << 1) + 1];
  const own = (val & T_FLAG) !== 0;
  const rep = val & ~T_FLAG;
  if (val === 0 || (scope & T_FLAG) !== 0) {
    throw new TypeError("Invalid handle");
  }
  table[handle << 1] = table[0] | T_FLAG;
  table[0] = handle | T_FLAG;
  return { rep, scope, own };
}

let curResourceBorrows = [];

function taskReturn(ctx) {
  const {
    componentIdx,
    getMemoryFn,
    memoryIdx,
    callbackFnIdx,
    liftFns,
    lowerFns,
    stringEncoding,
  } = ctx;
  const params = [...arguments].slice(1);
  const memory = getMemoryFn();
  let useDirectParams = ctx.useDirectParams;
  
  const { taskID } = _getGlobalCurrentTaskMeta(componentIdx);
  
  const taskMeta = getCurrentTask(componentIdx, taskID);
  if (!taskMeta) { throw new Error('failed to retrieve current task metadata'); }
  
  const task = taskMeta.task;
  if (!task) { throw new Error('invalid/missing current task in metadata'); }
  
  _debugLog('[taskReturn()] args', {
    componentIdx,
    taskID: task.id(),
    subtaskID: task.getParentSubtask()?.id(),
    callbackFnIdx,
    memoryIdx,
    liftFns,
    lowerFns,
    params,
  });
  
  // If we are in a subtask, and have a fused helper function provided to use
  // via PrepareCall, we can use that function rather than performing lifting manually.
  //
  // See also documentation on `HostIntrinsic::PrepareCall`
  const subtaskCallMetadata = task.getParentSubtask()?.getCallMetadata();
  if (subtaskCallMetadata?.returnFn && !subtaskCallMetadata.returnFnCalled) {
    _debugLog('[taskReturn()] calling return fn on subtask', {
      componentIdx,
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      returnFnParams: [...params, subtaskCallMetadata.resultPtr],
    });
    const res = subtaskCallMetadata.returnFn.apply(null, [...params, subtaskCallMetadata.resultPtr]);
    subtaskCallMetadata.returnFnCalled = true;
    task.resolve([]);
    return;
  }
  
  const expectedMemoryIdx = task.getReturnMemoryIdx();
  if (expectedMemoryIdx !== null && memoryIdx !== null && expectedMemoryIdx !== memoryIdx) {
    _debugLog("[taskReturn()] mismatched memory indices", { expectedMemoryIdx, memoryIdx });
    throw new Error('task.return memory [' + memoryIdx + '] does not match task [' + expectedMemoryIdx + ']');
  }
  
  task.callbackFnIdx = callbackFnIdx;
  
  if (!memory && liftFns.length > 4) {
    _debugLog("[taskReturn()] memory not present for max async flat lifts");
    throw new Error('memory must be present if more than max async flat lifts are performed');
  }
  
  let liftCtx = { memory, useDirectParams, params, componentIdx, stringEncoding };
  if (!useDirectParams) {
    if (!ctx.memory) {
      _debugLog('missing memory despite indirect param usage', { useDirectParams, liftCtx, ctx });
      throw new Error('missing memory despite indirect param usage');
    }
    liftCtx.storagePtr = params[0];
    liftCtx.storageLen = params[1];
  }
  
  const liftedResults = [];
  _debugLog('[taskReturn()] lifting results out of memory', { liftCtx });
  for (const liftFn of liftFns) {
    if (liftCtx.storageLen !== undefined && liftCtx.storageLen <= 0) {
      _debugLog(`[taskReturn()] ran out of range while writing storageLen = [${liftCtx.storageLen}]`);
      throw new Error('ran out of storage while writing');
    }
    const [ val, newLiftCtx ] = liftFn(liftCtx);
    liftCtx = newLiftCtx;
    liftedResults.push(val);
  }
  
  task.resolve(liftedResults);
}

function taskCancel(componentIdx) {
  _debugLog('[taskCancel()] args', { componentIdx, isAsync });
  
  const state = getOrCreateAsyncState(componentIdx);
  if (!state.mayLeave) { throw new Error('component instance is not marked as may leave, cannot be cancelled'); }
  
  const { taskID } = _getGlobalCurrentTaskMeta(componentIdx);
  
  const taskMeta = getCurrentTask(componentIdx, taskID);
  if (!taskMeta) { throw new Error('invalid/missing async task meta'); }
  
  const task = taskMeta.task;
  if (!task) { throw new Error('invalid/missing async task'); }
  
  if (task.sync && !task.alwaysTaskReturn) {
    throw new Error('cannot cancel sync tasks without always task return set');
  }
  
  task.cancel();
}

function createNewCurrentTask(args) {
  _debugLog('[createNewCurrentTask()] args', args);
  const {
    componentIdx,
    isAsync,
    isManualAsync,
    preserveFutureResult,
    entryFnName,
    parentSubtaskID,
    callbackFnName,
    getCallbackFn,
    getParamsFn,
    stringEncoding,
    errHandling,
    getCalleeParamsFn,
    resultPtr,
    callingWasmExport,
  } = args;
  if (componentIdx === undefined || componentIdx === null) {
    throw new Error('missing/invalid component instance index while starting task');
  }
  let taskMetas = ASYNC_TASKS_BY_COMPONENT_IDX.get(componentIdx);
  const callbackFn = getCallbackFn ? getCallbackFn() : null;
  
  const newTask = new AsyncTask({
    componentIdx,
    isAsync,
    isManualAsync,
    preserveFutureResult,
    entryFnName,
    callbackFn,
    callbackFnName,
    stringEncoding,
    getCalleeParamsFn,
    resultPtr,
    errHandling,
  });
  
  const newTaskID = newTask.id();
  const newTaskMeta = { id: newTaskID, componentIdx, task: newTask };
  
  // NOTE: do not track host tasks
  ASYNC_CURRENT_TASK_IDS.push(newTaskID);
  ASYNC_CURRENT_COMPONENT_IDXS.push(componentIdx);
  
  if (!taskMetas) {
    taskMetas = [newTaskMeta];
    ASYNC_TASKS_BY_COMPONENT_IDX.set(componentIdx, [newTaskMeta]);
  } else {
    taskMetas.push(newTaskMeta);
  }
  
  return [newTask, newTaskID];
}

async function _driverLoop(args) {
  _debugLog('[_driverLoop()] args', args);
  const {
    componentState,
    task,
    fnName,
    isAsync,
  } = args;
  let callbackResult = args.callbackResult;
  
  const callbackFnName = task.getCallbackFnName();
  const componentIdx = task.componentIdx();
  
  if (callbackResult instanceof Promise) {
    throw new Error("callbackResult should be a value, not a promise");
  }
  
  if (callbackResult === undefined) {
    throw new Error("callback result should never be undefined");
  }
  
  let callbackCode;
  let waitableSetRep;
  let unpacked;
  try {
    if (!(_typeCheckValidI32(callbackResult))) {
      throw new Error('invalid callback result [' + callbackResult + '], not a number');
    }
    
    unpacked = unpackCallbackResult(callbackResult);
    callbackCode = unpacked[0];
    waitableSetRep = unpacked[1];
  } catch(err) {
    console.error("failed to unpack callback result", err);
    throw err;
  }
  
  if (callbackCode < 0 || callbackCode > 3) {
    throw new Error('invalid async return value, outside callback code range');
  }
  
  const cstate = getOrCreateAsyncState(componentIdx);
  
  let eventCode;
  let index;
  let result;
  let asyncRes;
  let wset;
  try {
    while (true) {
      if (callbackCode !== 0) { componentState.exclusiveRelease(); }
      
      switch (callbackCode) {
        case 0: // EXIT
        _debugLog('[_driverLoop()] async exit indicated', {
          fnName,
          componentIdx,
          callbackFnName,
          taskID: task.id()
        });
        task.exit({ skipExclusiveLockCheck: true });
        return;
        
        case 1: // YIELD
        _debugLog('[_driverLoop()] yield', {
          fnName,
          componentIdx,
          callbackFnName,
          taskID: task.id()
        });
        asyncRes = await task.yieldUntil({
          cancellable: true,
          readyFn: () => !componentState.isExclusivelyLocked(),
        });
        _debugLog('[_driverLoop()] finished yield', {
          fnName,
          componentIdx,
          callbackFnName,
          taskID: task.id(),
          asyncRes,
        });
        break;
        
        case 2: // WAIT for a given waitable set
        _debugLog('[_driverLoop()] waiting for event', {
          fnName,
          componentIdx,
          callbackFnName,
          taskID: task.id(),
          waitableSetRep,
          waitableSetTargets: cstate.handles.get(waitableSetRep).targets(),
        });
        
        wset = cstate.handles.get(waitableSetRep);
        if (!(wset instanceof WaitableSet)) {
          throw new Error(`non-waitable set returned from component state handles @ [${waitableSetRep}]`);
        }
        
        asyncRes = await wset.waitUntil({
          readyFn: () => !componentState.isExclusivelyLocked(),
          task,
          cancellable: true,
        });
        
        _debugLog('[_driverLoop()] finished waiting for event', {
          fnName,
          componentIdx,
          callbackFnName,
          taskID: task.id(),
          waitableSetRep,
          asyncRes,
        });
        
        break;
        
        default:
        throw new Error(`Unrecognized async function result [${ret}]`);
      }
      
      componentState.exclusiveLock();
      
      // If the task failed via any means, leave early and reject.
      if (task.isRejected()) {
        _debugLog('[_driverLoop()] detected task rejection, leaving early');
        return;
      }
      
      if (asyncRes.code === undefined) { throw new Error("missing event code from event"); }
      if (asyncRes.payload0 === undefined) { throw new Error("missing payload0 from event"); }
      if (asyncRes.payload1 === undefined) { throw new Error("missing payload1 from event"); }
      
      eventCode = asyncRes.code; // async event enum code
      index = asyncRes.payload0; // varies (e.g. idx of related waitable set)
      result = asyncRes.payload1; // varies (e.g. task state)
      asyncRes = null;
      
      _debugLog('[_driverLoop()] performing callback', {
        fnName,
        componentIdx,
        taskID: task.id(),
        callbackFnName,
        eventCode,
        index,
        result
      });
      
      const callbackRes = await task.runCallbackFn(
      toInt32(eventCode),
      toInt32(index),
      toInt32(result),
      );
      
      unpacked = unpackCallbackResult(callbackRes);
      callbackCode = unpacked[0];
      waitableSetRep = unpacked[1];
      
      _debugLog('[_driverLoop()] callback result unpacked', {
        fnName,
        componentIdx,
        callbackFnName,
        callbackRes,
        callbackCode,
        waitableSetRep,
      });
    }
  } catch (err) {
    _debugLog('[_driverLoop()] error during async driver loop', {
      fnName,
      callbackFnName,
      componentIdx,
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      parentTaskID: task.getParentSubtask()?.getParentTask()?.id(),
      event: {
        eventCode,
        index,
        result,
      },
      err,
    });
    task.setErrored(err);
    task.reject(err);
  }
}

function _lowerImportBackwardsCompat(args) {
  const params = [...arguments].slice(1);
  _debugLog('[_lowerImportBackwardsCompat()] args', { args, params });
  const {
    functionIdx,
    componentIdx,
    isAsync,
    isManualAsync,
    paramLiftFns,
    resultLowerFns,
    hasResultPointer,
    funcTypeIsAsync,
    metadata,
    memoryIdx,
    getMemoryFn,
    getReallocFn,
    importFn,
    stringEncoding,
  } = args;
  
  let meta = _getGlobalCurrentTaskMeta(componentIdx);
  let createdTask;
  
  // Some components depend on initialization logic (i.e. `_initialize` or some such
  // core wasm export) that is embedded in the component, but is not executed or wizer'd
  // away before the transpiled component is attempted to be used.
  //
  // These components execut their initialization logic *when they are imported* in the
  // transpiled context -- so we may get a call to an export that is lowered without going
  // through `CallWasm` or `CallInterface`.
  //
  if (!meta) {
    if (funcTypeIsAsync || (isAsync && !isManualAsync)) {
      throw new Error('p3 async wasm exports cannot use backwards compat auto-task init');
    }
    
    const [newTask, newTaskID] = createNewCurrentTask({
      componentIdx,
      isAsync,
      isManualAsync,
      callingWasmExport: false,
    });
    createdTask = newTask;
    
    // Since we're managing the task creation ourselves we must clear ourselves
    createdTask.registerOnResolveHandler(() => {
      _clearCurrentTask({
        taskID: task.id(),
        componentIdx: task.componentIdx(),
      });
    });
    
    _setGlobalCurrentTaskMeta({
      componentIdx,
      taskID: newTaskID,
    });
    
    meta = _getGlobalCurrentTaskMeta(componentIdx);
  }
  
  const { taskID } = meta;
  
  const taskMeta = getCurrentTask(componentIdx, taskID);
  if (!taskMeta) {
    throw new Error('invalid/missing async task meta');
  }
  
  const task = taskMeta.task;
  if (!task) { throw new Error('invalid/missing async task'); }
  
  const cstate = getOrCreateAsyncState(componentIdx);
  
  // TODO: re-enable this check -- postReturn can call imports though,
  // and that breaks things.
  //
  // if (!cstate.mayLeave) {
    //     throw new Error(`cannot leave instance [${componentIdx}]`);
    // }
    
    if (!task.mayBlock() && funcTypeIsAsync && !isAsync) {
      throw new Error("non async exports cannot synchronously call async functions");
    }
    
    // If there is an existing task, this should be part of a subtask
    const memory = getMemoryFn();
    // Canonical ABI lower appends result storage as a trailing
    // param when async lower has any flat result, or sync lower
    // has more than one flat result.
    const resultPtr = hasResultPointer ? params[params.length - 1] : undefined;
    const subtask = task.createSubtask({
      componentIdx,
      parentTask: task,
      fnName: importFn.fnName,
      isAsync,
      isManualAsync,
      callMetadata: {
        memoryIdx,
        memory,
        realloc: getReallocFn?.(),
        getReallocFn,
        resultPtr,
        lowers: resultLowerFns,
        stringEncoding,
      }
    });
    task.setReturnMemoryIdx(memoryIdx);
    task.setReturnMemory(getMemoryFn());
    
    subtask.onStart();
    
    // If dealing with a sync lowered sync function, we can directly return results
    //
    // TODO(breaking): remove once we get rid of manual async import specification,
    // as func types cannot be detected in that case only (and we don't need that w/ p3)
    if (!isManualAsync && !isAsync && !funcTypeIsAsync) {
      if (createdTask) { createdTask.enterSync(); }
      
      const res = importFn(...params);
      
      // TODO(breaking): remove once we get rid of manual async import specification,
      // as func types cannot be detected in that case only (and we don't need that w/ p3)
      if (!funcTypeIsAsync && !subtask.isReturned()) {
        throw new Error('post-execution subtasks must either be async or returned');
      }
      
      const syncRes = subtask.getResult();
      if (createdTask) { createdTask.resolve([syncRes]); }
      
      return syncRes;
    }
    
    // Sync-lowered async functions requires async behavior because the callee *can* block,
    // but this call must *act* synchronously and return immediately with the result
    // (i.e. not returning until the work is done)
    //
    // TODO(breaking): remove checking for manual async specification here, once we can go p3-only
    //
    if (!isManualAsync && !isAsync && funcTypeIsAsync) {
      const { promise, resolve } = new Promise();
      queueMicrotask(async () => {
        if (!subtask.isResolvedState()) {
          await task.suspendUntil({ readyFn: () => task.isResolvedState() });
        }
        resolve(subtask.getResult());
      });
      return promise;
    }
    
    // NOTE: at this point we know that we are working with an async lowered import
    
    const subtaskState = subtask.getStateNumber();
    if (subtaskState < 0 || subtaskState >= 2**4) {
      throw new Error('invalid subtask state, out of valid range');
    }
    
    subtask.setOnProgressFn(() => {
      subtask.setPendingEvent(() => {
        if (subtask.isResolved()) { subtask.deliverResolve(); }
        const event = {
          code: ASYNC_EVENT_CODE.SUBTASK,
          payload0: subtask.waitableRep(),
          payload1: subtask.getStateNumber(),
        }
        return event;
      });
    });
    
    // This is a hack to maintain backwards compatibility with
    // manually-specified async imports, used in wasm exports that are
    // not actually async (but are specified as so).
    //
    // This is not normal p3 sync behavior but instead anticipating that
    // the caller that is doing manual async will be waiting for a promise that
    // resolves to the *actual* result.
    //
    // TODO(breaking): remove once manually specified async is removed
    //
    // There are a few cases:
    // 1. sync function with async types (e.g. `f: func() -> stream<u32>`)
    // 2. async function with async types (e.g. `f: async func() -> stream<u32>`)
    // 3. async function with sync types (e.g. `f: async func() -> list<u32>`)
    // 4. sync function with non-async types (e.g. `f: func() -> list<u32>`)
    //
    // This hack *only* applies to 4 -- the case where an async JS host function
    // is supplied to a Wasm export which does *not* need to do any async abi
    // lifting/lowering (async ABI did not exist when JSPI integratiton was
    // initially merged to enable asynchronously returning values from the host)
    //
    const requiresManualAsyncResult = !isAsync && !funcTypeIsAsync && isManualAsync;
    let manualAsyncResult;
    if (requiresManualAsyncResult) {
      manualAsyncResult = promiseWithResolvers();
    }
    
    queueMicrotask(async () => {
      try {
        _debugLog('[_lowerImportBackwardsCompat()] calling lowered import', { importFn, params });
        if (createdTask) { await createdTask.enter(); }
        
        const asyncRes = await importFn(...params);
        if (requiresManualAsyncResult) {
          manualAsyncResult.resolve(subtask.getResult());
        }
        
        if (createdTask) { createdTask.resolve([asyncRes]); }
        
        
      } catch (err) {
        _debugLog("[_lowerImportBackwardsCompat()] import fn error:", err);
        if (requiresManualAsyncResult) {
          manualAsyncResult.reject(err);
        }
        throw err;
      }
    });
    
    if (requiresManualAsyncResult) { return manualAsyncResult.promise; }
    
    return Number(subtask.waitableRep()) << 4 | subtaskState;
  }
  
  class WaitableSet {
    #componentIdx;
    #waitables = [];
    #pendingEvent = null;
    #waiting = 0;
    
    target;
    
    constructor(componentIdx) {
      if (componentIdx === undefined) { throw new TypeError("missing/invalid component idx"); }
      this.#componentIdx = componentIdx;
      this.target = `component [${this.#componentIdx}] waitable set`;
    }
    
    componentIdx() { return this.#componentIdx; }
    
    numWaitables() { return this.#waitables.length; }
    numWaiting() { return this.#waiting; }
    
    incrementNumWaiting(n) { this.#waiting += n ?? 1; }
    decrementNumWaiting(n) { this.#waiting -= n ?? 1; }
    
    targets() { return this.#waitables.map(w => w.target); }
    
    setTarget(tgt) { this.target = tgt; }
    
    shuffleWaitables() {
      this.#waitables = this.#waitables
      .map(value => ({ value, sort: Math.random() }))
      .sort((a, b) => a.sort - b.sort)
      .map(({ value }) => value);
    }
    
    removeWaitable(waitable) {
      const existing = this.#waitables.find(w => w === waitable);
      if (!existing) { return undefined; }
      this.#waitables = this.#waitables.filter(w => w !== waitable);
      return waitable;
    }
    
    addWaitable(waitable) {
      this.removeWaitable(waitable);
      this.#waitables.push(waitable);
    }
    
    hasPendingEvent() {
      _debugLog('[WaitableSet#hasPendingEvent()] args', {
        componentIdx: this.#componentIdx,
        waitableSet: this,
        waitableSetTargets: this.targets(),
      });
      const waitable = this.#waitables.find(w => w.hasPendingEvent());
      return waitable !== undefined;
    }
    
    getPendingEvent() {
      _debugLog('[WaitableSet#getPendingEvent()] args', {
        componentIdx: this.#componentIdx,
        waitableSet: this,
      });
      for (const waitable of this.#waitables) {
        if (!waitable.hasPendingEvent()) { continue; }
        const event = waitable.getPendingEvent();
        _debugLog('[WaitableSet#getPendingEvent()] found pending event', {
          waitable,
          event,
        });
        return event;
      }
      throw new Error('no waitables had a pending event');
    }
    
    async waitUntil(opts) {
      _debugLog('[WaitableSet#waitUntil()] args', { opts });
      // TODO(threads): this task should be the thread
      const { readyFn, task, cancellable } = opts;
      
      let event;
      
      this.incrementNumWaiting();
      
      const keepGoing = await task.suspendUntil({
        readyFn: () => {
          const hasPendingEvent = this.hasPendingEvent();
          const ready = readyFn();
          return ready && hasPendingEvent;
        },
        cancellable,
      });
      
      if (keepGoing) {
        event = this.getPendingEvent();
      } else {
        event = {
          code: ASYNC_EVENT_CODE.TASK_CANCELLED,
          payload0: 0,
          payload1: 0,
        };
      }
      
      this.decrementNumWaiting();
      
      return event;
    }
    
  }
  
  function waitableSetNew(componentIdx) {
    _debugLog('[waitableSetNew()] args', { componentIdx });
    
    const state = getOrCreateAsyncState(componentIdx);
    if (!state) {throw new Error(`missing async state for component idx [${componentIdx}]`); }
    
    const wset = new WaitableSet(componentIdx);
    const rep = state.handles.insert(wset);
    if (typeof rep !== 'number') { throw new Error(`invalid/missing waitable set rep [${rep}]`); }
    
    _debugLog('[waitableSetNew()] created waitable set', { componentIdx, rep });
    return rep;
  }
  
  function waitableSetPoll(ctx, waitableSetRep, resultPtr) {
    const { componentIdx, memoryIdx, getMemoryFn, isAsync, isCancellable } = ctx;
    _debugLog('[waitableSetPoll()] args', {
      componentIdx,
      memoryIdx,
      waitableSetRep,
      resultPtr,
    });
    
    const taskMeta = getCurrentTask(componentIdx);
    if (!taskMeta) { throw Error('invalid/missing current task meta'); }
    if (taskMeta.componentIdx !== componentIdx) {
      throw Error('task component idx [' + task.componentIdx + '] != component instance ID [' + componentIdx + ']');
    }
    
    const task = taskMeta.task;
    if (!task) { throw Error('invalid/missing async task in task meta'); }
    
    if (task.componentIdx() !== componentIdx) {
      throw Error(`task component idx [${task.componentIdx()}] does not match generated [${componentIdx}]`);
    }
    
    const cstate = getOrCreateAsyncState(task.componentIdx());
    const wset = cstate.handles.get(waitableSetRep);
    if (!wset) {
      throw new Error(`missing waitable set [${waitableSetRep}] in component [${componentIdx}]`);
    }
    
    let event;
    const cancelDelivered = task.deliverPendingCancel({ cancellable: isCancellable });
    if (cancelDelivered) {
      _debugLog('[waitableSetPoll()] detected cancel delivered', {
        componentIdx,
        waitableSetRep,
      });
      event = { code: ASYNC_EVENT_CODE.TASK_CANCELLED, payload0: 0, payload1: 0 };
    } else if (!wset.hasPendingEvent()) {
      _debugLog('[waitableSetPoll()] no pending event', {
        componentIdx,
        waitableSetRep,
      });
      event = { code: ASYNC_EVENT_CODE.NONE, payload0: 0, payload1: 0 };
    } else {
      _debugLog('[waitableSetPoll()] retrieving waiting pending event', {
        componentIdx,
        waitableSetRep,
      });
      event = wset.getPendingEvent();
    }
    
    const eventCode = _storeEventInComponentMemory({
      event,
      ptr: resultPtr,
      memory: getMemoryFn(),
      componentIdx,
      task,
      memoryIdx,
    });
    
    return eventCode;
  }
  
  function waitableSetDrop(componentIdx, waitableSetRep) {
    _debugLog('[waitableSetDrop()] args', { componentIdx, waitableSetRep });
    const task = getCurrentTask(componentIdx);
    
    if (!task) { throw new Error('invalid/missing async task'); }
    if (task.componentIdx !== componentIdx) {
      throw Error('task component idx [' + task.componentIdx + '] != component instance ID [' + componentIdx + ']');
    }
    
    const state = getOrCreateAsyncState(componentIdx);
    if (!state.mayLeave) { throw new Error('component instance is not marked as may leave, cannot be cancelled'); }
    
    _removeWaitableSet({ state, waitableSetRep });
  }
  
  function _removeWaitableSet(args) {
    _debugLog('[_removeWaitableSet()] args', args);
    const { state, waitableSetRep } = args;
    if (!state) { throw new TypeError("missing component state"); }
    if (!waitableSetRep) { throw new TypeError("missing component waitableSetRep"); }
    
    const ws = state.handles.get(waitableSetRep);
    if (!ws) {
      throw new Error('cannot remove waitable set: no set present with rep [' + waitableSetRep + ']');
    }
    if (ws.hasPendingEvent()) {
      throw new Error('waitable set cannot be removed with pending items remaining');
    }
    
    const waitableSet = state.handles.get(waitableSetRep);
    if (ws.numWaitables() > 0) {
      throw new Error('waitable set still contains waitables');
    }
    if (ws.numWaiting() > 0) {
      throw new Error('waitable set still has other tasks waiting on it');
    }
    
    state.handles.remove(waitableSetRep);
  }
  
  function waitableJoin(componentIdx, waitableRep, waitableSetRep) {
    _debugLog('[waitableJoin()] args', {
      componentIdx,
      waitableSetRep,
      isRemoval: waitableSetRep === 0,
      waitableRep,
    });
    
    const state = getOrCreateAsyncState(componentIdx);
    if (!state) {
      throw new Error(`invalid/missing async state for component instance [${componentIdx}]`);
    }
    
    if (!state.mayLeave) {
      throw new Error('component instance is not marked as may leave, cannot join waitable');
    }
    
    const waitableObj = state.handles.get(waitableRep);
    if (!waitableObj) {
      throw new Error(`missing waitable obj (rep [${waitableRep}]), component idx [${componentIdx}])`);
    }
    const waitable = waitableObj.getWaitable ? waitableObj.getWaitable() : waitableObj;
    if (!waitable.join) {
      throw new Error("invalid waitable object, does not have join()");
    }
    
    const waitableSet = waitableSetRep === 0 ? null : state.handles.get(waitableSetRep);
    if (waitableSetRep !== 0 && !waitableSet) {
      throw new Error(`missing waitable set [${waitableSetRep}] in component idx [${componentIdx}]`);
    }
    
    waitable.join(waitableSet);
  }
  
  function _liftFlatBool(ctx) {
    _debugLog('[_liftFlatBool()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least a single i32 argument'); }
      val = ctx.params[0] === 1;
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 1) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (bool requires 1 byte)`);
    }
    
    val = new DataView(ctx.memory.buffer).getUint8(ctx.storagePtr, true) === 1;
    
    ctx.storagePtr += 1;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 1; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatU8(ctx) {
    _debugLog('[_liftFlatU8()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least a single i32 argument'); }
      val = ctx.params[0];
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 1) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u8 requires 1 byte)`);
    }
    
    val = new DataView(ctx.memory.buffer).getUint8(ctx.storagePtr, true);
    
    ctx.storagePtr += 1;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 1; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatU16(ctx) {
    _debugLog('[_liftFlatU16()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least a single i32 argument'); }
      val = ctx.params[0];
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 2) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u16 requires 2 bytes)`);
    }
    
    val = new DataView(ctx.memory.buffer).getUint16(ctx.storagePtr, true);
    
    ctx.storagePtr += 2;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 2; }
    
    const rem = ctx.storagePtr % 2;
    if (rem !== 0) { ctx.storagePtr += (2 - rem); }
    
    return [val, ctx];
  }
  
  
  function _liftFlatU32(ctx) {
    _debugLog('[_liftFlatU32()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least a single i34 argument'); }
      val = ctx.params[0];
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 4) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u32 requires 4 bytes)`);
    }
    val = new DataView(ctx.memory.buffer).getUint32(ctx.storagePtr, true);
    ctx.storagePtr += 4;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 4; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatU64(ctx) {
    _debugLog('[_liftFlatU64()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) { throw new Error('expected at least one single i64 argument'); }
      if (typeof ctx.params[0] !== 'bigint') { throw new Error('expected bigint'); }
      val = ctx.params[0];
      ctx.params = ctx.params.slice(1);
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 8) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (u64 requires 8 bytes)`);
    }
    
    val = new DataView(ctx.memory.buffer).getBigUint64(ctx.storagePtr, true);
    ctx.storagePtr += 8;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 8; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatFloat64(ctx) {
    _debugLog('[_liftFlatFloat64()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length === 0) {
        throw new Error('expected at least one single f64 argument');
      }
      val = ctx.params[0];
      ctx.params = ctx.params.slice(1);
      
      if (ctx.inVariant) {
        const dv = new DataView(new ArrayBuffer(8));
        dv.setBigInt64(0, val);
        val = dv.getFloat64(0);
      }
      
      return [val, ctx];
    }
    
    if (ctx.storageLen !== undefined && ctx.storageLen < 8) {
      throw new Error(`insufficient storage ([${ctx.storageLen}] bytes) for lift (f64 requires 8 bytes)`);
    }
    
    val = new DataView(ctx.memory.buffer).getFloat64(ctx.storagePtr, true);
    ctx.storagePtr += 8;
    if (ctx.storageLen !== undefined) { ctx.storageLen -= 8; }
    
    return [val, ctx];
  }
  
  
  function _liftFlatStringAny(ctx) {
    switch (ctx.stringEncoding) {
      case 'utf8':
      return _liftFlatStringUTF8(ctx);
      case 'utf16':
      return _liftFlatStringUTF16(ctx);
      default:
      throw new Error(`missing/unrecognized/unsupported string encoding [${ctx.stringEncoding}]`);
    }
  }
  
  function _liftFlatStringUTF8(ctx) {
    _debugLog('[_liftFlatStringUTF8()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length < 2) { throw new Error('expected at least two u32 arguments'); }
      let offset = ctx.params[0];
      if (typeof offset === 'bigint') { offset = Number(offset); }
      if (!Number.isSafeInteger(offset)) { throw new Error('invalid offset'); }
      const len = ctx.params[1];
      if (!Number.isSafeInteger(len)) {  throw new Error('invalid len'); }
      val = TEXT_DECODER_UTF8.decode(new DataView(ctx.memory.buffer, offset, len));
      ctx.params = ctx.params.slice(2);
      return [val, ctx];
    }
    
    const rem = ctx.storagePtr % 4;
    if (rem !== 0) { ctx.storagePtr += (4 - rem); }
    
    const dv = new DataView(ctx.memory.buffer);
    const start = dv.getUint32(ctx.storagePtr, true);
    const codeUnits = dv.getUint32(ctx.storagePtr + 4, true);
    
    val = TEXT_DECODER_UTF8.decode(new Uint8Array(ctx.memory.buffer, start, codeUnits));
    
    ctx.storagePtr += 8;
    if (ctx.storageLen !== undefined) { ctx.storagelen -= 8; }
    
    return [val, ctx];
  }
  
  function _liftFlatStringUTF16(ctx) {
    _debugLog('[_liftFlatStringUTF16()] args', { ctx });
    let val;
    
    if (ctx.useDirectParams) {
      if (ctx.params.length < 2) { throw new Error('expected at least two u32 arguments'); }
      let offset = ctx.params[0];
      if (typeof offset === 'bigint') { offset = Number(offset); }
      if (!Number.isSafeInteger(offset)) {  throw new Error('invalid offset'); }
      const len = ctx.params[1];
      if (!Number.isSafeInteger(len)) {  throw new Error('invalid len'); }
      val = utf16Decoder.decode(new DataView(ctx.memory.buffer, offset, len));
      ctx.params = ctx.params.slice(2);
      return [val, ctx];
    }
    
    const data = new DataView(ctx.memory.buffer)
    const start = data.getUint32(ctx.storagePtr, vals[0], true);
    const codeUnits = data.getUint32(ctx.storagePtr, vals[0] + 4, true);
    val = utf16Decoder.decode(new Uint16Array(ctx.memory.buffer, start, codeUnits));
    ctx.storagePtr = ctx.storagePtr + 2 * codeUnits;
    if (ctx.storageLen !== undefined) { ctx.storageLen = ctx.storageLen - 2 * codeUnits }
    
    return [val, ctx];
  }
  
  function _liftFlatRecord(meta) {
    const { fieldMetas, size32: recordSize32, align32: recordAlign32 } = meta;
    return function _liftFlatRecordInner(ctx) {
      _debugLog('[_liftFlatRecord()] args', { ctx });
      
      const originalPtr = ctx.storagePtr;
      const res = {};
      for (const [key, liftFn, size32, align32] of fieldMetas) {
        let fieldPtr;
        if (ctx.storagePtr !== undefined) {
          const rem = ctx.storagePtr % align32;
          if (rem !== 0) { ctx.storagePtr += align32 - rem; }
          fieldPtr = ctx.storagePtr;
        }
        
        // A field occupies exactly size32 bytes of the record's
        // flat storage. Capture the remaining storage budget before
        // lifting the field and restore it afterwards: a field's own
        // lift fn may repurpose storageLen internally (e.g. a list
        // sets it to the element-buffer length while reading
        // out-of-line data and never restores it), which would
        // otherwise corrupt the budget the next field sees.
        // See https://github.com/bytecodealliance/jco/issues/1585.
        let fieldLen;
        if (ctx.storageLen !== undefined) { fieldLen = ctx.storageLen; }
        
        let [val, newCtx] = liftFn(ctx);
        res[key] = val;
        ctx = newCtx;
        
        if (fieldPtr !== undefined) {
          ctx.storagePtr = Math.max(ctx.storagePtr, fieldPtr + size32);
        }
        if (fieldLen !== undefined) {
          ctx.storageLen = fieldLen - size32;
        }
      }
      
      if (originalPtr !== undefined) {
        ctx.storagePtr = Math.max(ctx.storagePtr, originalPtr + recordSize32);
      }
      
      if (ctx.storagePtr !== undefined) {
        const rem = ctx.storagePtr % recordAlign32;
        if (rem !== 0) { ctx.storagePtr += recordAlign32 - rem; }
      }
      
      return [res, ctx];
    }
  }
  
  function _liftFlatVariant(meta) {
    const {
      caseMetas,
      variantSize32,
      variantAlign32,
      variantPayloadOffset32,
      variantFlatCount,
      isEnum,
    } = meta;
    
    return function _liftFlatVariantInner(ctx) {
      _debugLog('[_liftFlatVariant()] args', { ctx });
      const origUseParams = ctx.useDirectParams;
      
      // If we're in the process of lifting a variant, we note
      // we are during any lifting that happens (e.g. to accomodate f32/f64 mechanics)
      const wasInVariant = ctx.inVariant;
      ctx.inVariant = true;
      
      let caseIdx;
      let liftRes;
      const originalPtr = ctx.storagePtr;
      const numCases =  caseMetas.length;
      if (caseMetas.length < 256) {
        liftRes = _liftFlatU8(ctx);
      } else if (numCases >= 256 && numCases < 65536) {
        liftRes = _liftFlatU16(ctx);
      } else if (numCases >= 65536 && numCases < 4_294_967_296) {
        liftRes = _liftFlatU32(ctx);
      } else {
        throw new Error(`unsupported number of variant cases [${numCases}]`);
      }
      caseIdx = liftRes[0];
      ctx = liftRes[1];
      
      const [
      tag,
      liftFn,
      caseSize32,
      caseAlign32,
      caseFlatCount,
      ] = caseMetas[caseIdx];
      
      if (variantPayloadOffset32 === undefined) {
        throw new Error('unexpectedly missing payload offset');
      }
      
      if (originalPtr !== undefined) {
        ctx.storagePtr = originalPtr + variantPayloadOffset32;
      }
      
      let val;
      if (liftFn === null) {
        val = { tag };
        // NOTE: here we need to move past the entire object in memory
        // despite moving to the payload which we now know is missing/unnecessary
        if (originalPtr !== undefined) {
          ctx.storagePtr = originalPtr + variantSize32;
        }
      } else {
        if (ctx.useDirectParams && ctx.params && liftFn !== _liftFlatFloat64 && typeof ctx.params[0] === 'bigint') {
          if (ctx.params[0] > BigInt(Number.MAX_SAFE_INTEGER)) {
            throw new Error(`invalid value, reinterpreted i32/f32 too large: [${ctx.params[0]}]`);
          }
          ctx.params[0] = Number(ctx.params[0]);
        }
        
        const [newVal, newCtx] = liftFn(ctx);
        val = { tag, val: newVal };
        ctx = newCtx;
      }
      
      if (origUseParams) {
        if (variantFlatCount === undefined || variantFlatCount === null) {
          _debugLog('[_liftFlatVariant()] variant with unknown flat count', { ctx, meta });
          throw new Error('cannot lift variant with unknown flat count');
        }
        if (caseFlatCount === undefined || caseFlatCount === null) {
          _debugLog('[_liftFlatVariant()] case with unknown flat count', { ctx, meta, case: meta.caseMetas[caseIdx] });
          throw new Error('cannot lift case with unknown flat count');
        }
        // NOTE: enums can be tightly packed and do not have a descriminant
        const remainingPayloadParams = variantFlatCount - caseFlatCount - (isEnum ? 0 : 1);
        if (remainingPayloadParams < 0) {
          throw new Error(`invalid variant flat count metadata`);
        }
        if (ctx.params.length < remainingPayloadParams) {
          throw new Error(`expected at least [${remainingPayloadParams}] remaining variant payload params, but got [${ctx.params.length}]`);
        }
        ctx.params = ctx.params.slice(remainingPayloadParams);
      }
      
      if (ctx.storagePtr !== undefined) {
        const rem = ctx.storagePtr % variantAlign32;
        if (rem !== 0) { ctx.storagePtr += variantAlign32 - rem; }
      }
      
      ctx.inVariant = wasInVariant;
      
      return [val, ctx];
    }
  }
  
  function _liftFlatList(meta) {
    const { elemLiftFn, elemSize32, elemAlign32, knownLen, typedArray } = meta;
    
    const listValue =
    typedArray === undefined
    ? values => values
    : values => new typedArray(values);
    
    const readValuesAndReset = (ctx, originalPtr, originalLen, dataPtr, len) => {
      ctx.storagePtr = dataPtr;
      const val = [];
      for (var i = 0; i < len; i++) {
        const elemPtr = dataPtr + i * elemSize32;
        ctx.storagePtr = elemPtr;
        const [res, nextCtx] = elemLiftFn(ctx);
        val.push(res);
        ctx = nextCtx;
        
        ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + elemSize32);
      }
      if (originalPtr !== null) { ctx.storagePtr = originalPtr; }
      if (originalLen !== null) { ctx.storageLen = originalLen; }
      return [listValue(val), ctx];
    };
    
    return function _liftFlatListInner(ctx) {
      _debugLog('[_liftFlatList()] args', { ctx });
      
      let liftResults;
      if (knownLen !== undefined) { // list with known length
      if (ctx.useDirectParams) {
        _debugLog('memory unexpectedly missing while lifting unknown length list', { ctx });
        liftResults = [listValue(ctx.params.slice(0, knownLen)), ctx];
        ctx.params = ctx.params.slice(knownLen);
      } else { // indirect params
      if (ctx.memory === null) {
        _debugLog('memory unexpectedly missing while lifting known length list', { knownLen, ctx });
        throw new Error(`memory missing while lifting known length (${knownLen}) list`);
      }
      
      const originalLen = ctx.storageLen;
      const originalPtr = ctx.storagePtr;
      
      ctx.storageLen = knownLen * elemSize32;
      liftResults = readValuesAndReset(ctx, null, originalLen, ctx.storagePtr, knownLen);
    }
    
  } else { // unknown length list
  
  if (ctx.useDirectParams) {
    // unknown length list ptr w/ direct params
    const dataPtr = ctx.params[0];
    const len = ctx.params[1];
    ctx.params = ctx.params.slice(2);
    
    ctx.useDirectParams = false;
    const originalPtr = ctx.storagePtr;
    const originalLen = ctx.storageLen;
    ctx.storageLen = len * elemSize32;
    
    liftResults = readValuesAndReset(ctx, originalPtr, originalLen, dataPtr, len);
    
    ctx.useDirectParams = true;
  } else {
    // unknown length list ptr w/ in-memory params
    const originalLen = ctx.storageLen;
    ctx.storageLen = 8;
    
    const dataPtrLiftRes = _liftFlatU32(ctx);
    const dataPtr = dataPtrLiftRes[0];
    ctx = dataPtrLiftRes[1];
    
    const lenLiftRes = _liftFlatU32(ctx);
    const len = lenLiftRes[0];
    ctx = lenLiftRes[1];
    
    const originalPtr = ctx.storagePtr;
    ctx.storagePtr = dataPtr;
    
    ctx.storageLen = len * elemSize32;
    liftResults = readValuesAndReset(ctx, originalPtr, originalLen, dataPtr, len);
  }
}

return liftResults;
}
}

function _liftFlatTuple(meta) {
  const { elemLiftFns, size32: tupleSize32, align32: tupleAlign32 } = meta;
  return function _liftFlatTupleInner(ctx) {
    _debugLog('[_liftFlatTuple()] args', { ctx });
    
    const originalPtr = ctx.storagePtr;
    const val = [];
    for (const [ liftFn, size32, align32 ]  of elemLiftFns) {
      let elemPtr;
      if (ctx.storagePtr !== undefined) {
        const rem = ctx.storagePtr % align32;
        if (rem !== 0) { ctx.storagePtr += align32 - rem; }
        elemPtr = ctx.storagePtr;
      }
      
      // As in _liftFlatRecord: an element occupies exactly size32
      // bytes of the tuple's flat storage, so capture and restore
      // the storage budget around the element lift to stop a
      // field's internal storageLen use (e.g. lists) leaking into
      // the next element.
      // See https://github.com/bytecodealliance/jco/issues/1585.
      let elemLen;
      if (ctx.storageLen !== undefined) { elemLen = ctx.storageLen; }
      
      const [newValue, newCtx] = liftFn(ctx);
      val.push(newValue);
      ctx = newCtx;
      
      if (elemPtr !== undefined) {
        ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + size32);
      }
      if (elemLen !== undefined) {
        ctx.storageLen = elemLen - size32;
      }
    }
    
    if (originalPtr !== undefined) {
      ctx.storagePtr = Math.max(ctx.storagePtr, originalPtr + tupleSize32);
    }
    
    if (ctx.storagePtr !== undefined) {
      const rem = ctx.storagePtr % tupleAlign32;
      if (rem !== 0) { ctx.storagePtr += tupleAlign32 - rem; }
    }
    
    return [val, ctx];
  }
}

function _liftFlatEnum(meta) {
  meta.isEnum = true;
  const f = _liftFlatVariant(meta);
  return function _liftFlatEnumInner(ctx) {
    _debugLog('[_liftFlatEnum()] args', { ctx });
    const res = f(ctx);
    res[0] = res[0].tag;
    return res;
  }
}

function _liftFlatOption(meta) {
  const f = _liftFlatVariant(meta);
  return function _liftFlatOptionInner(ctx) {
    _debugLog('[_liftFlatOption()] args', { ctx });
    return f(ctx);
  }
}

function _liftFlatResult(meta) {
  const f = _liftFlatVariant(meta);
  return function _liftFlatResultInner(ctx) {
    _debugLog('[_liftFlatResult()] args', { ctx });
    return f(ctx);
  }
}

function _liftFlatBorrow(componentTableIdx, size, memory, vals, storagePtr, storageLen) {
  _debugLog('[_liftFlatBorrow()] args', { size, memory, vals, storagePtr, storageLen });
  throw new Error('flat lift for borrowed resources is not supported!');
}


function _lowerFlatBool(ctx) {
  _debugLog('[_lowerFlatBool()] args', { ctx });
  
  if (!ctx.memory) { throw new Error("missing memory for lower"); }
  if (ctx.vals.length !== 1) {
    throw new Error(`unexpected number [${ctx.vals.length}] of vals (expected 1)`);
  }
  
  _requireValidNumericPrimitive.bind('bool', ctx.vals[0]);
  new DataView(ctx.memory.buffer).setUint8(ctx.storagePtr, ctx.vals[0] ? 1 : 0);
  
  ctx.storagePtr += 1;
}

function _lowerFlatU8(ctx) {
  _debugLog('[_lowerFlatU8()] args', ctx);
  
  if (ctx.vals.length !== 1) {
    throw new Error(`unexpected number [${ctx.vals.length}] of vals (expected 1)`);
  }
  
  _requireValidNumericPrimitive.bind('u8', ctx.vals[0]);
  
  if (!ctx.memory) { throw new Error("missing memory for lower"); }
  new DataView(ctx.memory.buffer).setUint8(ctx.storagePtr, ctx.vals[0]);
  
  ctx.storagePtr += 1;
}

function _lowerFlatU16(ctx) {
  _debugLog('[_lowerFlatU16()] args', { ctx });
  
  if (!ctx.memory) { throw new Error("missing memory for lower"); }
  if (ctx.vals.length !== 1) {
    throw new Error(`unexpected number [${ctx.vals.length}] of vals (expected 1)`);
  }
  
  const rem = ctx.storagePtr % 2;
  if (rem !== 0) { ctx.storagePtr += (2 - rem); }
  
  _requireValidNumericPrimitive.bind('u16', ctx.vals[0]);
  new DataView(ctx.memory.buffer).setUint16(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 2;
}

function _lowerFlatU32(ctx) {
  _debugLog('[_lowerFlatU32()] args', { ctx });
  
  if (ctx.vals.length !== 1) {
    throw new Error(`expected single value to lower, got [${ctx.vals.length}]`);
  }
  
  const rem = ctx.storagePtr % 4;
  if (rem !== 0) { ctx.storagePtr += (4 - rem); }
  
  _requireValidNumericPrimitive.bind('u32', ctx.vals[0]);
  new DataView(ctx.memory.buffer).setUint32(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 4;
}

function _lowerFlatS64(ctx) {
  _debugLog('[_lowerFlatS64()] args', { ctx });
  
  if (ctx.vals.length !== 1) { throw new Error('unexpected number of vals'); }
  
  const rem = ctx.storagePtr % 8;
  if (rem !== 0) { ctx.storagePtr += (8 - rem); }
  
  _requireValidNumericPrimitive.bind('s64', ctx.vals[0]);
  new DataView(ctx.memory.buffer).setBigInt64(ctx.storagePtr, ctx.vals[0], true);
  
  
  ctx.storagePtr += 8;
}

function _lowerFlatU64(ctx) {
  _debugLog('[_lowerFlatU64()] args', { ctx });
  
  if (ctx.vals.length !== 1) { throw new Error('unexpected number of vals'); }
  
  const rem = ctx.storagePtr % 8;
  if (rem !== 0) { ctx.storagePtr += (8 - rem); }
  
  _requireValidNumericPrimitive.bind('u64', ctx.vals[0]);
  new DataView(ctx.memory.buffer).setBigUint64(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 8;
}

function _lowerFlatFloat64(ctx) {
  _debugLog('[_lowerFlatFloat64()] args', { ctx });
  
  if (ctx.vals.length !== 1) { throw new Error('unexpected number of vals'); }
  
  const rem = ctx.storagePtr % 8;
  if (rem !== 0) { ctx.storagePtr += (8 - rem); }
  
  _requireValidNumericPrimitive.bind('f64', ctx.vals[0]);
  new DataView(ctx.memory.buffer).setFloat64(ctx.storagePtr, ctx.vals[0], true);
  
  ctx.storagePtr += 8;
}

function _lowerFlatStringAny(ctx) {
  switch (ctx.stringEncoding) {
    case 'utf8':
    return _lowerFlatStringUTF8(ctx);
    case 'utf16':
    return _lowerFlatStringUTF16(ctx);
    default:
    throw new Error(`missing/unrecognized/unsupported string encoding [${ctx.stringEncoding}]`);
  }
}

function _lowerFlatStringUTF8(ctx) {
  _debugLog('[_lowerFlatStringUTF8()] args', ctx);
  if (!ctx.realloc) { throw new Error('missing realloc during flat string lower'); }
  
  const s = ctx.vals[0];
  const { ptr, codepoints } = _utf8AllocateAndEncode(ctx.vals[0], ctx.realloc, ctx.memory);
  
  const view = new DataView(ctx.memory.buffer);
  view.setUint32(ctx.storagePtr, ptr, true);
  view.setUint32(ctx.storagePtr + 4, codepoints, true);
  
  ctx.storagePtr += 8;
}

function _lowerFlatStringUTF16(ctx) {
  _debugLog('[_lowerFlatStringUTF16()] args', { ctx });
  if (!ctx.realloc) { throw new Error('missing realloc during flat string lower'); }
  
  const s = ctx.vals[0];
  const { ptr, len, codepoints } = _utf16AllocateAndEncode(ctx.vals[0], ctx.realloc, ctx.memory);
  
  const view = new DataView(ctx.memory.buffer);
  view.setUint32(ctx.storagePtr, ptr, true);
  view.setUint32(ctx.storagePtr + 4, codepoints, true);
  
  const bytes = new Uint16Array(ctx.memory.buffer, start, codeUnits);
  if (ctx.memory.buffer.byteLength < start + bytes.byteLength) {
    throw new Error('memory out of bounds');
  }
  if (ctx.storageLen !== undefined && ctx.storageLen !== bytes.byteLength) {
    throw new Error(`storage length [${ctx.storageLen}] != [${bytes.byteLength}])`);
  }
  new Uint16Array(ctx.memory.buffer, ctx.storagePtr).set(bytes);
  
  ctx.storagePtr += len;
}

function _lowerFlatRecord(meta) {
  const { fieldMetas, size32: recordSize32, align32: recordAlign32 } = meta;
  return function _lowerFlatRecordInner(ctx) {
    _debugLog('[_lowerFlatRecord()] args', { ctx });
    
    const originalPtr = ctx.storagePtr;
    const r = ctx.vals[0];
    for (const [tag, lowerFn, size32, align32 ] of fieldMetas) {
      const rem = ctx.storagePtr % align32;
      if (rem !== 0) { ctx.storagePtr += align32 - rem; }
      
      const fieldPtr = ctx.storagePtr;
      ctx.vals = [r[tag]];
      lowerFn(ctx);
      
      ctx.storagePtr = Math.max(ctx.storagePtr, fieldPtr + size32);
    }
    
    ctx.storagePtr = Math.max(ctx.storagePtr, originalPtr + recordSize32);
    
    const rem = ctx.storagePtr % recordAlign32;
    if (rem !== 0) {
      ctx.storagePtr += recordAlign32 - rem;
    }
  }
}

function _lowerFlatVariant(meta) {
  const { variantSize32, variantAlign32, variantPayloadOffset32, caseMetas } = meta;
  
  let caseLookup = {};
  for (const [idx, meta] of caseMetas.entries()) {
    let tag = meta[0];
    caseLookup[tag] = { discriminant: idx, meta };
  }
  
  return function _lowerFlatVariantInner(ctx) {
    _debugLog('[_lowerFlatVariant()] args', { ctx });
    
    const { tag, val } = ctx.vals[0];
    const variantCase = caseLookup[tag];
    if (!variantCase) {
      throw new Error(`missing tag [${tag}] (valid tags: ${Object.keys(caseLookup)})`);
    }
    
    const [ _tag, lowerFn, caseSize32, caseAlign32, caseFlatCount ] = variantCase.meta;
    
    const originalPtr = ctx.storagePtr;
    ctx.vals = [variantCase.discriminant];
    let discLowerRes;
    if (caseMetas.length < 256) {
      discLowerRes = _lowerFlatU8(ctx);
    } else if (caseMetas.length >= 256 && caseMetas.length < 65536) {
      discLowerRes = _lowerFlatU16(ctx);
    } else if (caseMetas.length >= 65536 && caseMetas.length < 4_294_967_296) {
      discLowerRes = _lowerFlatU32(ctx);
    } else {
      throw new Error(`unsupported number of cases [${caseMetas.length}]`);
    }
    
    const payloadOffsetPtr = originalPtr + variantPayloadOffset32;
    ctx.storagePtr = payloadOffsetPtr;
    ctx.vals = [val];
    if (lowerFn) { lowerFn(ctx); }
    
    ctx.storagePtr = Math.max(ctx.storagePtr, originalPtr + variantSize32);
    
    const rem = ctx.storagePtr % variantAlign32;
    if (rem !== 0) { ctx.storagePtr += varianttAlign32 - rem; }
  }
}

function _lowerFlatList(meta) {
  const {
    elemLowerFn,
    knownLen,
    size32,
    align32,
    elemSize32,
    elemAlign32,
  } = meta;
  
  if (!elemLowerFn) { throw new TypeError("missing/invalid element lower fn for list"); }
  
  return function _lowerFlatListInner(ctx) {
    _debugLog('[_lowerFlatList()] args', { ctx });
    
    if (ctx.useDirectParams) {
      if (ctx.params.length < 2) { throw new Error('insufficient params left to lower list'); }
      const storagePtr = ctx.params[0];
      const elemCount = ctx.params[1];
      ctx.params = ctx.params.slice(2);
      
      const list = ctx.vals[0];
      if (!list) { throw new Error("missing direct param value"); }
      
      const lowerCtx = {
        storagePtr,
        memory: ctx.memory,
        stringEncoding: ctx.stringEncoding,
      };
      for (let idx = 0; idx < list.length; idx++) {
        const elemPtr = storagePtr + idx * elemSize32;
        lowerCtx.storagePtr = elemPtr;
        lowerCtx.vals = list.slice(idx, idx+1);
        elemLowerFn(lowerCtx);
        lowerCtx.storagePtr = Math.max(lowerCtx.storagePtr, elemPtr + elemSize32);
      }
      ctx.storagePtr = lowerCtx.storagePtr;
      
      // TODO: implement parma-only known-length processing
      
      return;
    }
    
    // TODO(fix): is it possible to get a vals that are a addr and length here from
    // a component lower?
    
    const elems = ctx.vals[0];
    if (knownLen === undefined) {
      // unknown length
      if (!ctx.realloc) { throw new Error('missing realloc during flat string lower'); }
      const dataPtr = ctx.realloc(0, 0, elemAlign32, elemSize32 * elems.length);
      
      ctx.vals[0] = dataPtr;
      _lowerFlatU32(ctx);
      
      ctx.vals[0] = elems.length;
      _lowerFlatU32(ctx);
      
      const origPtr = ctx.storagePtr;
      ctx.storagePtr = dataPtr;
      
      for (const [idx, elem] of elems.entries()) {
        const elemPtr = dataPtr + idx * elemSize32;
        ctx.storagePtr = elemPtr;
        ctx.vals = [elem];
        elemLowerFn(ctx);
        ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + elemSize32);
      }
      
      ctx.storagePtr = origPtr;
      
    } else {
      // known length
      
      if (elems.length !== knownLen) {
        throw new TypeError(`invalid list input of length [${elems.length}], must be length [${knownLen}]`);
      }
      
      const originalPtr = ctx.storagePtr;
      for (const [idx, elem] of elems.entries()) {
        const elemPtr = originalPtr + idx * elemSize32;
        ctx.storagePtr = elemPtr;
        ctx.vals = [elem];
        elemLowerFn(ctx);
        ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + elemSize32);
      }
    }
    
    // TODO(fix): special case for u8/u16/etc, we can do a direct copy
    
    const totalSizeBytes = elems.length * size32;
    if (ctx.storageLen !== undefined && totalSizeBytes > ctx.storageLen) {
      throw new Error('not enough storage remaining for list flat lower');
    }
  }
}

function _lowerFlatTuple(meta) {
  const { elemLowerMetas, size32: tupleSize32, align32: tupleAlign32 } = meta;
  return function _lowerFlatTupleInner(ctx) {
    _debugLog('[_lowerFlatTuple()] args', { ctx });
    const originalPtr = ctx.storagePtr;
    const tuple = ctx.vals[0];
    for (const [idx, [ lowerFn, size32, align32 ]]  of elemLowerMetas.entries()) {
      const rem = ctx.storagePtr % align32;
      if (rem !== 0) { ctx.storagePtr += align32 - rem; }
      
      const elemPtr = ctx.storagePtr;
      ctx.vals = [tuple[idx]];
      lowerFn(ctx);
      ctx.storagePtr = Math.max(ctx.storagePtr, elemPtr + size32);
    }
    
    ctx.storagePtr = Math.max(ctx.storagePtr, originalPtr + tupleSize32);
    
    const rem = ctx.storagePtr % tupleAlign32;
    if (rem !== 0) {
      ctx.storagePtr += tupleAlign32 - rem;
    }
  }
}

function _lowerFlatEnum(meta) {
  const f = _lowerFlatVariant(meta);
  return function _lowerFlatEnumInner(ctx) {
    _debugLog('[_lowerFlatEnum()] args', { ctx });
    
    const v = ctx.vals[0];
    const isNotEnumObject = typeof v !== 'object'
    || Object.keys(v).length !== 2
    || !('tag' in v);
    if (isNotEnumObject) {
      ctx.vals[0] = { tag: v };
    }
    
    f(ctx);
  }
}

function _lowerFlatOption(meta) {
  const f = _lowerFlatVariant(meta);
  return function _lowerFlatOptionInner(ctx) {
    _debugLog('[_lowerFlatOption()] args', { ctx });
    
    const v = ctx.vals[0];
    if (v === null || v === undefined) {
      ctx.vals[0] = { tag: 'none' };
    } else {
      const isNotOptionObject = typeof v !== 'object'
      || Object.keys(v).length !== 2
      || !('tag' in v)
      || !(v.tag === 'some' || v.tag === 'none')
      || !('val' in v);
      if (isNotOptionObject) {
        ctx.vals[0] = { tag: 'some', val: v };
      }
    }
    
    f(ctx);
  }
}

function _lowerFlatResult(meta) {
  const f = _lowerFlatVariant(meta);
  return function _lowerFlatResultInner(ctx) {
    _debugLog('[_lowerFlatResult()] args', { ctx });
    
    const v = ctx.vals[0];
    const isNotResultObject = typeof v !== 'object'
    || Object.keys(v).length !== 2
    || !('tag' in v)
    || !('ok' === v.tag || 'err' === v.tag)
    || !('val' in v);
    if (isNotResultObject) {
      ctx.vals[0] = { tag: 'ok', val: v };
    }
    
    f(ctx);
  };
}

function _lowerFlatOwn(meta) {
  const { lowerFn, componentIdx } = meta;
  
  return function _lowerFlatOwnInner(ctx) {
    _debugLog('[_lowerFlatOwn()] args', { ctx });
    const { createFn } = ctx;
    
    if (ctx.componentIdx !== componentIdx) {
      throw new Error(`component index mismatch (expected [${componentIdx}], lift called from [${ctx.componentIdx}])`);
    }
    
    const obj = ctx.vals[0];
    if (obj === undefined || obj === null) { throw new Error('missing resource'); }
    const handle = lowerFn(obj);
    
    ctx.vals[0] = handle;
    _lowerFlatU32(ctx);
  };
}

const STREAMS = new RepTable({ target: 'global stream map' });
const ASYNC_STATE = new Map();

function getOrCreateAsyncState(componentIdx, init) {
  if (!ASYNC_STATE.has(componentIdx)) {
    const newState = new ComponentAsyncState({ componentIdx });
    ASYNC_STATE.set(componentIdx, newState);
  }
  return ASYNC_STATE.get(componentIdx);
}

class ComponentAsyncState {
  static EVENT_HANDLER_EVENTS = [ 'backpressure-change' ];
  
  #componentIdx;
  #callingAsyncImport = false;
  #syncImportWait = promiseWithResolvers();
  #locked = false;
  #parkedTasks = new Map();
  #suspendedTasksByTaskID = new Map();
  #suspendedTaskIDs = [];
  #errored = null;
  
  #backpressure = 0;
  #backpressureWaiters = 0n;
  
  #handlerMap = new Map();
  #nextHandlerID = 0n;
  
  #tickLoop = null;
  #tickLoopInterval = null;
  
  #onExclusiveReleaseHandlers = [];
  
  mayLeave = true;
  
  handles;
  subtasks;
  
  constructor(args) {
    this.#componentIdx = args.componentIdx;
    this.handles = new RepTable({ target: `component [${this.#componentIdx}] handles (waitable objects)` });
    this.subtasks = new RepTable({ target: `component [${this.#componentIdx}] subtasks` });
  };
  
  componentIdx() { return this.#componentIdx; }
  
  errored() { return this.#errored !== null; }
  setErrored(err) {
    _debugLog('[ComponentAsyncState#setErrored()] component errored', { err, componentIdx: this.#componentIdx });
    if (this.#errored) { return; }
    if (!err) {
      err = new Error('error elswehere (see other component instance error)')
      err.componentIdx = this.#componentIdx;
    }
    this.#errored = err;
  }
  
  callingSyncImport(val) {
    if (val === undefined) { return this.#callingAsyncImport; }
    if (typeof val !== 'boolean') { throw new TypeError('invalid setting for async import'); }
    const prev = this.#callingAsyncImport;
    this.#callingAsyncImport = val;
    if (prev === true && this.#callingAsyncImport === false) {
      this.#notifySyncImportEnd();
    }
  }
  
  #notifySyncImportEnd() {
    const existing = this.#syncImportWait;
    this.#syncImportWait = promiseWithResolvers();
    existing.resolve();
  }
  
  async waitForSyncImportCallEnd() {
    await this.#syncImportWait.promise;
  }
  
  setBackpressure(v) {
    this.#backpressure = v;
    return this.#backpressure
  }
  getBackpressure() { return this.#backpressure; }
  
  incrementBackpressure() {
    const current = this.#backpressure;
    if (current < 0 || current > 2**16) {
      throw new Error(`invalid current backpressure value [${current}]`);
    }
    const newValue = this.getBackpressure() + 1;
    if (newValue >= 2**16) {
      throw new Error(`invalid new backpressure value [${newValue}], overflow`);
    }
    return this.setBackpressure(newValue);
  }
  
  decrementBackpressure() {
    const current = this.#backpressure;
    if (current < 0 || current > 2**16) {
      throw new Error(`invalid current backpressure value [${current}]`);
    }
    const newValue = Math.max(0, current - 1);
    if (newValue < 0) {
      throw new Error(`invalid new backpressure value [${newValue}], underflow`);
    }
    return this.setBackpressure(newValue);
  }
  hasBackpressure() { return this.#backpressure > 0; }
  
  waitForBackpressure() {
    let backpressureCleared = false;
    const cstate = this;
    cstate.addBackpressureWaiter();
    const handlerID = this.registerHandler({
      event: 'backpressure-change',
      fn: (bp) => {
        if (bp === 0) {
          cstate.removeHandler(handlerID);
          backpressureCleared = true;
        }
      }
    });
    return new Promise((resolve) => {
      const interval = setInterval(() => {
        if (backpressureCleared) { return; }
        clearInterval(interval);
        cstate.removeBackpressureWaiter();
        resolve(null);
      }, 0);
    });
  }
  
  registerHandler(args) {
    const { event, fn } = args;
    if (!event) { throw new Error("missing handler event"); }
    if (!fn) { throw new Error("missing handler fn"); }
    
    if (!ComponentAsyncState.EVENT_HANDLER_EVENTS.includes(event)) {
      throw new Error(`unrecognized event handler [${event}]`);
    }
    
    const handlerID = this.#nextHandlerID++;
    let handlers = this.#handlerMap.get(event);
    if (!handlers) {
      handlers = [];
      this.#handlerMap.set(event, handlers)
    }
    
    handlers.push({ id: handlerID, fn, event });
    return handlerID;
  }
  
  removeHandler(args) {
    const { event, handlerID } = args;
    const registeredHandlers = this.#handlerMap.get(event);
    if (!registeredHandlers) { return; }
    const found = registeredHandlers.find(h => h.id === handlerID);
    if (!found) { return; }
    this.#handlerMap.set(event, this.#handlerMap.get(event).filter(h => h.id !== handlerID));
  }
  
  getBackpressureWaiters() { return this.#backpressureWaiters; }
  addBackpressureWaiter() { this.#backpressureWaiters++; }
  removeBackpressureWaiter() {
    this.#backpressureWaiters--;
    if (this.#backpressureWaiters < 0) {
      throw new Error("unexepctedly negative number of backpressure waiters");
    }
  }
  
  isExclusivelyLocked() { return this.#locked === true; }
  setLocked(locked) {
    this.#locked = locked;
  }
  
  exclusiveLock() {
    _debugLog('[ComponentAsyncState#exclusiveLock()]', {
      locked: this.#locked,
      componentIdx: this.#componentIdx,
    });
    this.setLocked(true);
  }
  
  exclusiveRelease() {
    _debugLog('[ComponentAsyncState#exclusiveRelease()] args', {
      locked: this.#locked,
      componentIdx: this.#componentIdx,
    });
    this.setLocked(false);
    
    this.#onExclusiveReleaseHandlers = this.#onExclusiveReleaseHandlers.filter(v => !!v);
    for (const [idx, f] of this.#onExclusiveReleaseHandlers.entries()) {
      try {
        this.#onExclusiveReleaseHandlers[idx] = null;
        f();
      } catch (err) {
        _debugLog("error while executing handler for next exclusive release", err);
        throw err;
      }
    }
  }
  
  onNextExclusiveRelease(fn) {
    _debugLog('[ComponentAsyncState#()onNextExclusiveRelease] registering');
    this.#onExclusiveReleaseHandlers.push(fn);
  }
  
  // nextTaskPromise & nextTaskQueue are used to await current task completion and queues
  // any tasks attempting to enter() and complete.
  //
  // see: nextTaskExecutionSlot()
  //
  // TODO(threads): this should be unnecessary once threads are properly implemented,
  // as the task.enter() logic should suffice (it should be guaranteed that we cannot re-enter
  // unless the task in question is the current task in the thread execution, and only one can
  // run at a time)
  #nextTaskPromise = Promise.resolve(true);
  #nextTaskQueue = [];
  
  async nextTaskExecutionSlot(args) {
    const { task } = args;
    
    const placeholder = {
      completed: false,
      task,
      promise: task.exitPromise().then(() => {
        placeholder.completed = true;
      }),
    };
    this.#nextTaskQueue.push(placeholder);
    
    let next;
    while (true) {
      await this.#nextTaskPromise;
      
      next = this.#nextTaskQueue.find(placeholder => !placeholder.completed);
      
      // This task is next in the queue, we can continue
      if (next === undefined || next === placeholder) {
        this.#nextTaskPromise = next.promise;
        if (this.#nextTaskQueue.length > 1000) {
          this.#nextTaskQueue = this.#nextTaskQueue.filter(p => !p.completed);
          if (this.#nextTaskQueue.length > 1000) {
            _debugLog('[ComponentAsyncState#()nextTaskExecutionSlot] next task queue length > 1000 even after cleanup, tasks may be leaking');
          }
        }
        break;
      }
      
      // If we get here, this task was *not* next in the queue, continue waiting
      // (at this point the task that *is* next will likely have already set itself
      // as this.#nextTaskPromise)
    }
  }
  
  #getSuspendedTaskMeta(taskID) {
    return this.#suspendedTasksByTaskID.get(taskID);
  }
  
  #removeSuspendedTaskMeta(taskID) {
    _debugLog('[ComponentAsyncState#removeSuspendedTaskMeta()] removing suspended task', {
      taskID,
      componentIdx: this.#componentIdx,
    });
    const idx = this.#suspendedTaskIDs.findIndex(t => t === taskID);
    const meta = this.#suspendedTasksByTaskID.get(taskID);
    this.#suspendedTaskIDs[idx] = null;
    this.#suspendedTasksByTaskID.delete(taskID);
    return meta;
  }
  
  #addSuspendedTaskMeta(meta) {
    if (!meta) { throw new Error('missing task meta'); }
    const taskID = meta.taskID;
    this.#suspendedTasksByTaskID.set(taskID, meta);
    this.#suspendedTaskIDs.push(taskID);
    if (this.#suspendedTasksByTaskID.size < this.#suspendedTaskIDs.length - 10) {
      this.#suspendedTaskIDs = this.#suspendedTaskIDs.filter(t => t !== null);
    }
  }
  
  // TODO(threads): readyFn is normally on the thread
  suspendTask(args) {
    const { task, readyFn } = args;
    const taskID = task.id();
    const componentIdx = task.componentIdx();
    _debugLog('[ComponentAsyncState#suspendTask()]', {
      taskID,
      componentIdx: this.#componentIdx,
      taskEntryFnName: task.entryFnName(),
      subtask: task.getParentSubtask(),
    });
    
    if (componentIdx !== this.#componentIdx) {
      throw new Error('assert: task component idx should match async state');
    }
    
    if (this.#getSuspendedTaskMeta(taskID)) {
      throw new Error(`task [${taskID}] already suspended`);
    }
    
    const { promise, resolve, reject } = promiseWithResolvers();
    this.#addSuspendedTaskMeta({
      task,
      taskID,
      readyFn,
      resume: () => {
        _debugLog('[ComponentAsyncState] resuming suspended task', {
          taskID,
          componentIdx: this.#componentIdx,
        });
        // TODO(threads): it's thread cancellation we should be checking for below, not task
        resolve(!task.isCancelled());
      },
    });
    
    this.runTickLoop();
    
    return promise;
  }
  
  resumeTaskByID(taskID) {
    const meta = this.#removeSuspendedTaskMeta(taskID);
    if (!meta) { return; }
    if (meta.taskID !== taskID) { throw new Error('task ID does not match'); }
    meta.resume();
  }
  
  async runTickLoop() {
    if (this.#tickLoop !== null) { return; }
    this.#tickLoop = 1;
    setTimeout(async () => {
      let done = this.tick();
      while (!done) {
        await new Promise((resolve) => setTimeout(resolve, 30));
        done = this.tick();
      }
      this.#tickLoop = null;
    }, 10);
  }
  
  tick() {
    // _debugLog('[ComponentAsyncState#tick()]', { suspendedTaskIDs: this.#suspendedTaskIDs });
    
    const resumableTasks = this.#suspendedTaskIDs.filter(t => t !== null);
    for (const taskID of resumableTasks) {
      const meta = this.#suspendedTasksByTaskID.get(taskID);
      if (!meta || !meta.readyFn) {
        throw new Error(`missing/invalid task despite ID [${taskID}] being present`);
      }
      
      // If the task failed via any means, allow the task to resume because
      // it's been cancelled -- the callback should immediately exit as well
      if (meta.task.isRejected()) {
        _debugLog('[ComponentAsyncState#tick()] detected task rejection, leaving early', { meta });
        this.resumeTaskByID(taskID);
        return;
      }
      
      const isReady = meta.readyFn();
      if (!isReady) { continue; }
      
      _debugLog('[ComponentAsyncState#tick()] resuming task via tick', {
        taskID,
        componentIdx: this.#componentIdx,
      });
      this.resumeTaskByID(taskID);
    }
    
    return this.#suspendedTaskIDs.filter(t => t !== null).length === 0;
  }
  
  addStreamEndToTable(args) {
    _debugLog('[ComponentAsyncState#addStreamEnd()] args', args);
    const { tableIdx, streamEnd } = args;
    if (typeof streamEnd === 'number') { throw new Error("INSERTING BAD STREAMEND"); }
    
    let { table, componentIdx } = STREAM_TABLES[tableIdx];
    if (componentIdx === undefined || !table) {
      throw new Error(`invalid global stream table state for table [${tableIdx}]`);
    }
    
    const handle = table.insert(streamEnd);
    streamEnd.setHandle(handle);
    streamEnd.setStreamTableIdx(tableIdx);
    
    const cstate = getOrCreateAsyncState(componentIdx);
    const waitableIdx = cstate.handles.insert(streamEnd);
    streamEnd.setWaitableIdx(waitableIdx);
    
    _debugLog('[ComponentAsyncState#addStreamEnd()] added stream end', {
      tableIdx,
      table,
      handle,
      streamEnd,
      destComponentIdx: componentIdx,
    });
    
    return { handle, waitableIdx };
  }
  
  createWaitable(args) {
    return new Waitable({ target: args?.target, });
  }
  
  createReadableStreamEnd(args) {
    _debugLog('[ComponentAsyncState#createStreamEnd()] args', args);
    const { tableIdx, elemMeta, hostInjectFn } = args;
    
    const { table: localStreamTable, componentIdx } = STREAM_TABLES[tableIdx];
    if (!localStreamTable) {
      throw new Error(`missing global stream table lookup for table [${tableIdx}] while creating stream`);
    }
    if (componentIdx !== this.#componentIdx) {
      throw new Error('component idx mismatch while creating stream');
    }
    
    const waitable = this.createWaitable();
    const streamEnd = new StreamReadableEnd({
      tableIdx,
      elemMeta,
      hostInjectFn,
      pendingBufferMeta: {},
      target: `stream read end (lowered, @init)`,
      waitable,
    });
    
    streamEnd.setWaitableIdx(this.handles.insert(streamEnd));
    streamEnd.setHandle(localStreamTable.insert(streamEnd));
    if (streamEnd.streamTableIdx() !== tableIdx) {
      throw new Error("unexpectedly mismatched stream table");
    }
    const streamEndWaitableIdx = streamEnd.waitableIdx();
    const streamEndHandle = streamEnd.handle();
    waitable.setTarget(`waitable for stream read end (lowered, waitable [${streamEndWaitableIdx}])`);
    streamEnd.setTarget(`stream read end (lowered, waitable [${streamEndWaitableIdx}])`);
    
    return {
      waitableIdx: streamEndWaitableIdx,
      handle: streamEndHandle,
      streamEnd,
    };
  }
  
  createStream(args) {
    _debugLog('[ComponentAsyncState#createStream()] args', args);
    const { tableIdx, elemMeta, hostInjectFn } = args;
    if (tableIdx === undefined) { throw new Error("missing table idx while adding stream"); }
    if (elemMeta === undefined) { throw new Error("missing element metadata while adding stream"); }
    
    const { table: localStreamTable, componentIdx } = STREAM_TABLES[tableIdx];
    if (!localStreamTable) {
      throw new Error(`missing global stream table lookup for table [${tableIdx}] while creating stream`);
    }
    if (componentIdx !== this.#componentIdx) {
      throw new Error('component idx mismatch while creating stream');
    }
    
    const readWaitable = this.createWaitable();
    const writeWaitable = this.createWaitable();
    
    const stream = new InternalStream({
      tableIdx,
      elemMeta,
      readWaitable,
      writeWaitable,
      hostInjectFn,
    });
    stream.setGlobalStreamMapRep(STREAMS.insert(stream));
    
    const writeEnd = stream.writeEnd();
    writeEnd.setWaitableIdx(this.handles.insert(writeEnd));
    writeEnd.setHandle(localStreamTable.insert(writeEnd));
    if (writeEnd.streamTableIdx() !== tableIdx) { throw new Error("unexpectedly mismatched stream table"); }
    
    const writeEndWaitableIdx = writeEnd.waitableIdx();
    const writeEndHandle = writeEnd.handle();
    writeWaitable.setTarget(`waitable for stream write end (waitable [${writeEndWaitableIdx}])`);
    writeEnd.setTarget(`stream write end (waitable [${writeEndWaitableIdx}])`);
    
    const readEnd = stream.readEnd();
    readEnd.setWaitableIdx(this.handles.insert(readEnd));
    readEnd.setHandle(localStreamTable.insert(readEnd));
    if (readEnd.streamTableIdx() !== tableIdx) { throw new Error("unexpectedly mismatched stream table"); }
    
    const readEndWaitableIdx = readEnd.waitableIdx();
    const readEndHandle = readEnd.handle();
    readWaitable.setTarget(`waitable for read end (waitable [${readEndWaitableIdx}])`);
    readEnd.setTarget(`stream read end (waitable [${readEndWaitableIdx}])`);
    
    return {
      writeEnd,
      writeEndWaitableIdx,
      writeEndHandle,
      readEndWaitableIdx,
      readEndHandle,
      readEnd,
    };
  }
  
  getStreamEnd(args) {
    _debugLog('[ComponentAsyncState#getStreamEnd()] args', args);
    const { tableIdx, streamEndHandle, streamEndWaitableIdx } = args;
    if (tableIdx === undefined) {
      throw new Error('missing table idx while getting stream end');
    }
    
    const { table, componentIdx } = STREAM_TABLES[tableIdx];
    const cstate = getOrCreateAsyncState(componentIdx);
    
    let streamEnd;
    if (streamEndWaitableIdx !== undefined) {
      streamEnd = cstate.handles.get(streamEndWaitableIdx);
    } else if (streamEndHandle !== undefined) {
      if (!table) { throw new Error(`missing/invalid table [${tableIdx}] while getting stream end`); }
      streamEnd = table.get(streamEndHandle);
    } else {
      throw new TypeError("must specify either waitable idx or handle to retrieve stream");
    }
    
    if (!streamEnd) {
      throw new Error(`missing stream end (tableIdx [${tableIdx}], handle [${streamEndHandle}], waitableIdx [${streamEndWaitableIdx}])`);
    }
    if (tableIdx && streamEnd.streamTableIdx() !== tableIdx) {
      throw new Error(`stream end table idx [${streamEnd.streamTableIdx()}] does not match [${tableIdx}]`);
    }
    
    return streamEnd;
  }
  
  deleteStreamEnd(args) {
    _debugLog('[ComponentAsyncState#deleteStreamEnd()] args', args);
    const { tableIdx, streamEndWaitableIdx } = args;
    if (tableIdx === undefined) { throw new Error("missing table idx while removing stream end"); }
    if (streamEndWaitableIdx === undefined) { throw new Error("missing stream idx while removing stream end"); }
    
    const { table, componentIdx } = STREAM_TABLES[tableIdx];
    const cstate = getOrCreateAsyncState(componentIdx);
    
    const streamEnd = cstate.handles.get(streamEndWaitableIdx);
    if (!streamEnd) {
      throw new Error(`missing stream end [${streamEndWaitableIdx}] in component handles while deleting stream`);
    }
    if (streamEnd.streamTableIdx() !== tableIdx) {
      throw new Error(`stream end table idx [${streamEnd.streamTableIdx()}] does not match [${tableIdx}]`);
    }
    
    let removed = cstate.handles.remove(streamEnd.waitableIdx());
    if (!removed) {
      throw new Error(`failed to remove stream end [${streamEndWaitableIdx}] waitable obj in component [${componentIdx}]`);
    }
    
    removed = table.remove(streamEnd.handle());
    if (!removed) {
      throw new Error(`failed to remove stream end with handle [${streamEnd.handle()}] from stream table [${tableIdx}] in component [${componentIdx}]`);
    }
    
    return streamEnd;
  }
  
  removeStreamEndFromTable(args) {
    _debugLog('[ComponentAsyncState#removeStreamEndFromTable()] args', args);
    
    const { tableIdx, streamWaitableIdx } = args;
    if (tableIdx === undefined) { throw new Error("missing table idx while removing stream end"); }
    if (streamWaitableIdx === undefined) {
      throw new Error("missing stream end waitable idx while removing stream end");
    }
    
    const { table, componentIdx } = STREAM_TABLES[tableIdx];
    if (!table) { throw new Error(`missing/invalid table [${tableIdx}] while removing stream end`); }
    
    const cstate = getOrCreateAsyncState(componentIdx);
    
    const streamEnd = cstate.handles.get(streamWaitableIdx);
    if (!streamEnd) {
      throw new Error(`missing stream end (handle [${streamWaitableIdx}], table [${tableIdx}])`);
    }
    const handle = streamEnd.handle();
    
    let removed = cstate.handles.remove(streamWaitableIdx);
    if (!removed) {
      throw new Error(`failed to remove streamEnd from handles (waitable idx [${streamWaitableIdx}]), component [${componentIdx}])`);
    }
    
    removed = table.remove(handle);
    if (!removed) {
      throw new Error(`failed to remove streamEnd from table (handle [${handle}]), table [${tableIdx}], component [${componentIdx}])`);
    }
    
    return streamEnd;
  }
  
  createFuture(args) {
    _debugLog('[ComponentAsyncState#createFuture()] args', args);
    const { tableIdx, elemMeta, hostInjectFn } = args;
    if (tableIdx === undefined) { throw new Error("missing table idx while adding future"); }
    if (elemMeta === undefined) { throw new Error("missing element metadata while adding future"); }
    
    const { table: futureTable, componentIdx } = FUTURE_TABLES[tableIdx];
    if (!futureTable) {
      throw new Error(`missing global future table lookup for table [${tableIdx}] while creating future`);
    }
    if (componentIdx !== this.#componentIdx) {
      throw new Error('component idx mismatch while creating future');
    }
    
    const readWaitable = this.createWaitable();
    const writeWaitable = this.createWaitable();
    
    const future = new InternalFuture({
      tableIdx,
      componentIdx: this.#componentIdx,
      elemMeta,
      readWaitable,
      writeWaitable,
      hostInjectFn,
    });
    future.setGlobalFutureMapRep(FUTURES.insert(future));
    
    const writeEnd = future.writeEnd();
    writeEnd.setWaitableIdx(this.handles.insert(writeEnd));
    writeEnd.setHandle(futureTable.insert(writeEnd));
    if (writeEnd.futureTableIdx() !== tableIdx) { throw new Error("unexpectedly mismatched future table"); }
    
    const writeEndWaitableIdx = writeEnd.waitableIdx();
    const writeEndHandle = writeEnd.handle();
    writeWaitable.setTarget(`waitable for future write end (waitable [${writeEndWaitableIdx}])`);
    writeEnd.setTarget(`future write end (waitable [${writeEndWaitableIdx}])`);
    
    const readEnd = future.readEnd();
    readEnd.setWaitableIdx(this.handles.insert(readEnd));
    readEnd.setHandle(futureTable.insert(readEnd));
    if (readEnd.futureTableIdx() !== tableIdx) { throw new Error("unexpectedly mismatched future table"); }
    
    const readEndWaitableIdx = readEnd.waitableIdx();
    const readEndHandle = readEnd.handle();
    readWaitable.setTarget(`waitable for read end (waitable [${readEndWaitableIdx}])`);
    readEnd.setTarget(`future read end (waitable [${readEndWaitableIdx}])`);
    
    return {
      writeEnd,
      writeEndWaitableIdx,
      writeEndHandle,
      readEndWaitableIdx,
      readEndHandle,
      readEnd,
    };
  }
  
  getFutureEnd(args) {
    _debugLog('[ComponentAsyncState#getFutureEnd()] args', args);
    const { tableIdx, futureEndHandle, futureEndWaitableIdx } = args;
    if (tableIdx === undefined) {
      throw new Error('missing table idx while getting future end');
    }
    
    const { table, componentIdx } = FUTURE_TABLES[tableIdx];
    const cstate = getOrCreateAsyncState(componentIdx);
    
    let futureEnd;
    if (futureEndWaitableIdx !== undefined) {
      futureEnd = cstate.handles.get(futureEndWaitableIdx);
    } else if (futureEndHandle !== undefined) {
      if (!table) { throw new Error(`missing/invalid table [${tableIdx}] while getting future end`); }
      futureEnd = table.get(futureEndHandle);
    } else {
      throw new TypeError("must specify either waitable idx or handle to retrieve future");
    }
    
    if (!futureEnd) {
      throw new Error(`missing future end (tableIdx [${tableIdx}], handle [${futureEndHandle}], waitableIdx [${futureEndWaitableIdx}])`);
    }
    if (tableIdx && futureEnd.futureTableIdx() !== tableIdx) {
      throw new Error(`future end table idx [${futureEnd.futureTableIdx()}] does not match [${tableIdx}]`);
    }
    
    return futureEnd;
  }
  
  removeFutureEndFromTable(args) {
    _debugLog('[ComponentAsyncState#removeFutureEndFromTable()] args', args);
    
    const { tableIdx, futureWaitableIdx } = args;
    if (tableIdx === undefined) { throw new Error("missing table idx while removing future end"); }
    if (futureWaitableIdx === undefined) {
      throw new Error("missing future end waitable idx while removing future end");
    }
    
    const { table, componentIdx } = FUTURE_TABLES[tableIdx];
    if (!table) { throw new Error(`missing/invalid table [${tableIdx}] while removing future end`); }
    
    const cstate = getOrCreateAsyncState(componentIdx);
    
    const futureEnd = cstate.handles.get(futureWaitableIdx);
    if (!futureEnd) {
      throw new Error(`missing future end (handle [${futureWaitableIdx}], table [${tableIdx}])`);
    }
    const handle = futureEnd.handle();
    
    let removed = cstate.handles.remove(futureWaitableIdx);
    if (!removed) {
      throw new Error(`failed to remove futureEnd from handles (waitable idx [${futureWaitableIdx}]), component [${componentIdx}])`);
    }
    
    removed = table.remove(handle);
    if (!removed) {
      throw new Error(`failed to remove futureEnd from table (handle [${handle}]), table [${tableIdx}], component [${componentIdx}])`);
    }
    
    return futureEnd;
  }
  
}

function _ComponentStateSetAllError() {
  _debugLog('[_ComponentStateSetAllError()]');
  for (const state of ASYNC_STATE.values()) {
    state.setErrored();
  }
}

function _storeEventInComponentMemory(args) {
  _debugLog('[_storeEventInComponentMemory()] args', args);
  const { memory, ptr, event } = args;
  
  if (!memory) { throw new Error('unexpectedly missing memory'); }
  if (ptr === undefined || ptr === null) { throw new Error('unexpectedly missing pointer'); }
  if (!event) { throw new Error('event object missing'); }
  if (event.code === undefined) { throw new Error('invalid event object, missing code'); }
  if (event.payload0 === undefined) { throw new Error('invalid event object, missing payload0'); }
  if (event.payload1 === undefined) { throw new Error('invalid event object, missing payload1'); }
  
  const dv = new DataView(memory.buffer);
  dv.setUint32(ptr, event.payload0, true);
  dv.setUint32(ptr + 4, event.payload1, true);
  
  return event.code;
}

const base64Compile = str => WebAssembly.compile(
typeof Buffer !== 'undefined'
? Buffer.from(str, 'base64')
: Uint8Array.from(atob(str), b => b.charCodeAt(0))
);


const isNode = typeof process !== 'undefined' && process.versions && process.versions.node;
let _fs;
async function fetchCompile (url) {
  if (isNode) {
    _fs = _fs || await import('node:fs/promises');
    return WebAssembly.compile(await _fs.readFile(url));
  }
  return fetch(url).then(WebAssembly.compileStreaming);
}

const symbolCabiDispose = Symbol.for('cabiDispose');

const symbolRscHandle = Symbol('handle');

const symbolRscRep = Symbol.for('cabiRep');

const HANDLE_TABLES= [];


function getErrorPayload(e) {
  if (e && hasOwnProperty.call(e, 'payload')) return e.payload;
  if (e instanceof Error) throw e;
  return e;
}

const isLE = new Uint8Array(new Uint16Array([1]).buffer)[0] === 1;

const hasOwnProperty = Object.prototype.hasOwnProperty;

const instantiateCore = WebAssembly.instantiate;


let exports0;

const _trampoline1 = function() {
  _debugLog('[iface="semio:framework/pure@1.0.0", function="now-ms"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'nowMs',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => nowMs(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="semio:framework/pure@1.0.0", function="now-ms"][Instruction::Return]', {
    funcName: 'now-ms',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([toInt64(ret)]);
  task.exit();
  return toInt64(ret);
}
_trampoline1.fnName = 'semio:framework/pure@1.0.0#nowMs';

const _trampoline12 = function(arg0) {
  let variant0;
  switch (arg0) {
    case 0: {
      variant0= {
        tag: 'ok',
        val: undefined
      };
      break;
    }
    case 1: {
      variant0= {
        tag: 'err',
        val: undefined
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  _debugLog('[iface="wasi:cli/exit@0.2.9", function="exit"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'exit',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => exit(variant0),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  _debugLog('[iface="wasi:cli/exit@0.2.9", function="exit"][Instruction::Return]', {
    funcName: 'exit',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline12.fnName = 'wasi:cli/exit@0.2.9#exit';

const handleTable0 = [T_FLAG, 0];
handleTable0._createdReps = new Set();


const captureTable0= new Map();
let captureCnt0= 0;

HANDLE_TABLES[0] = handleTable0;

const _trampoline13 = function(arg0) {
  var handle1 = arg0;
  
  var rep2 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
  var rsc0 = captureTable0.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(Pollable.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  _debugLog('[iface="wasi:io/poll@0.2.9", function="[method]pollable.block"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'block',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => rsc0.block(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  for (const entry of curResourceBorrows) {
    const rsc = entry.rsc ?? entry;
    if (entry.drop) {
      if (rsc[symbolRscHandle]) {
        entry.drop(rsc[symbolRscHandle]);
      }
    }
    rsc[symbolRscHandle] = undefined;
  }
  curResourceBorrows = [];
  _debugLog('[iface="wasi:io/poll@0.2.9", function="[method]pollable.block"][Instruction::Return]', {
    funcName: '[method]pollable.block',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline13.fnName = 'wasi:io/poll@0.2.9#block';

const handleTable3 = [T_FLAG, 0];
handleTable3._createdReps = new Set();


const captureTable3= new Map();
let captureCnt3= 0;

HANDLE_TABLES[3] = handleTable3;

const _trampoline14 = function(arg0) {
  var handle1 = arg0;
  
  var rep2 = handleTable3[(handle1 << 1) + 1] & ~T_FLAG;
  var rsc0 = captureTable3.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(OutputStream.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.subscribe"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'subscribe',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => rsc0.subscribe(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  for (const entry of curResourceBorrows) {
    const rsc = entry.rsc ?? entry;
    if (entry.drop) {
      if (rsc[symbolRscHandle]) {
        entry.drop(rsc[symbolRscHandle]);
      }
    }
    rsc[symbolRscHandle] = undefined;
  }
  curResourceBorrows = [];
  
  if (!(ret instanceof Pollable)) {
    throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
  }
  var handle3 = ret[symbolRscHandle];
  if (!handle3) {
    const rep = ret[symbolRscRep] || ++captureCnt0;
    captureTable0.set(rep, ret);
    handle3 = rscTableCreateOwn(handleTable0, rep);
  }
  
  _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.subscribe"][Instruction::Return]', {
    funcName: '[method]output-stream.subscribe',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([handle3]);
  task.exit();
  return handle3;
}
_trampoline14.fnName = 'wasi:io/streams@0.2.9#subscribe';

const handleTable2 = [T_FLAG, 0];
handleTable2._createdReps = new Set();


const captureTable2= new Map();
let captureCnt2= 0;

HANDLE_TABLES[2] = handleTable2;

const _trampoline15 = function() {
  _debugLog('[iface="wasi:cli/stdin@0.2.9", function="get-stdin"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getStdin',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getStdin(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  if (!(ret instanceof InputStream)) {
    throw new TypeError('Resource error: Not a valid \"InputStream\" resource.');
  }
  var handle0 = ret[symbolRscHandle];
  if (!handle0) {
    const rep = ret[symbolRscRep] || ++captureCnt2;
    captureTable2.set(rep, ret);
    handle0 = rscTableCreateOwn(handleTable2, rep);
  }
  
  _debugLog('[iface="wasi:cli/stdin@0.2.9", function="get-stdin"][Instruction::Return]', {
    funcName: 'get-stdin',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([handle0]);
  task.exit();
  return handle0;
}
_trampoline15.fnName = 'wasi:cli/stdin@0.2.9#getStdin';

const _trampoline16 = function() {
  _debugLog('[iface="wasi:cli/stdout@0.2.9", function="get-stdout"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getStdout',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getStdout(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  if (!(ret instanceof OutputStream)) {
    throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
  }
  var handle0 = ret[symbolRscHandle];
  if (!handle0) {
    const rep = ret[symbolRscRep] || ++captureCnt3;
    captureTable3.set(rep, ret);
    handle0 = rscTableCreateOwn(handleTable3, rep);
  }
  
  _debugLog('[iface="wasi:cli/stdout@0.2.9", function="get-stdout"][Instruction::Return]', {
    funcName: 'get-stdout',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([handle0]);
  task.exit();
  return handle0;
}
_trampoline16.fnName = 'wasi:cli/stdout@0.2.9#getStdout';

const _trampoline17 = function() {
  _debugLog('[iface="wasi:cli/stderr@0.2.9", function="get-stderr"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getStderr',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getStderr(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  if (!(ret instanceof OutputStream)) {
    throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
  }
  var handle0 = ret[symbolRscHandle];
  if (!handle0) {
    const rep = ret[symbolRscRep] || ++captureCnt3;
    captureTable3.set(rep, ret);
    handle0 = rscTableCreateOwn(handleTable3, rep);
  }
  
  _debugLog('[iface="wasi:cli/stderr@0.2.9", function="get-stderr"][Instruction::Return]', {
    funcName: 'get-stderr',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([handle0]);
  task.exit();
  return handle0;
}
_trampoline17.fnName = 'wasi:cli/stderr@0.2.9#getStderr';

const _trampoline18 = function(arg0) {
  _debugLog('[iface="wasi:clocks/monotonic-clock@0.2.9", function="subscribe-duration"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'subscribeDuration',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => subscribeDuration(BigInt.asUintN(64, BigInt(arg0))),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  
  if (!(ret instanceof Pollable)) {
    throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
  }
  var handle0 = ret[symbolRscHandle];
  if (!handle0) {
    const rep = ret[symbolRscRep] || ++captureCnt0;
    captureTable0.set(rep, ret);
    handle0 = rscTableCreateOwn(handleTable0, rep);
  }
  
  _debugLog('[iface="wasi:clocks/monotonic-clock@0.2.9", function="subscribe-duration"][Instruction::Return]', {
    funcName: 'subscribe-duration',
    paramCount: 1,
    async: false,
    postReturn: false
  });
  task.resolve([handle0]);
  task.exit();
  return handle0;
}
_trampoline18.fnName = 'wasi:clocks/monotonic-clock@0.2.9#subscribeDuration';
let exports1;
let memory0;
let realloc0;
let realloc0Async;

const _trampoline25 = function(arg0) {
  _debugLog('[iface="wasi:random/insecure-seed@0.2.9", function="insecure-seed"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'insecureSeed',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => insecureSeed(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var [tuple0_0, tuple0_1] = ret;
  dataView(memory0).setBigInt64(arg0 + 0, toUint64(tuple0_0), true);
  dataView(memory0).setBigInt64(arg0 + 8, toUint64(tuple0_1), true);
  _debugLog('[iface="wasi:random/insecure-seed@0.2.9", function="insecure-seed"][Instruction::Return]', {
    funcName: 'insecure-seed',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline25.fnName = 'wasi:random/insecure-seed@0.2.9#insecureSeed';

const _trampoline26 = function(arg0, arg1, arg2) {
  var len3 = arg1;
  var base3 = arg0;
  var result3 = [];
  for (let i = 0; i < len3; i++) {
    const base = base3 + i * 4;
    var handle1 = dataView(memory0).getInt32(base + 0, true);
    
    var rep2 = handleTable0[(handle1 << 1) + 1] & ~T_FLAG;
    var rsc0 = captureTable0.get(rep2);
    if (!rsc0) {
      rsc0 = Object.create(Pollable.prototype);
      Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
      Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
    }
    
    curResourceBorrows.push(rsc0);
    result3.push(rsc0);
  }
  _debugLog('[iface="wasi:io/poll@0.2.9", function="poll"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'poll',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => poll(result3),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  for (const entry of curResourceBorrows) {
    const rsc = entry.rsc ?? entry;
    if (entry.drop) {
      if (rsc[symbolRscHandle]) {
        entry.drop(rsc[symbolRscHandle]);
      }
    }
    rsc[symbolRscHandle] = undefined;
  }
  curResourceBorrows = [];
  var val4 = ret;
  var len4 = val4.length;
  var ptr4 = realloc0(0, 0, 4, len4 * 4);
  
  let valData4;
  const valLenBytes4 = len4 * 4;
  if (Array.isArray(val4)) {
    // Regular array likely containing numbers, write values to memory
    let offset = 0;
    const dv4 = new DataView(memory0.buffer);
    for (const v of val4) {
      _requireValidNumericPrimitive.bind(null, 'u32')(v);
      dv4.setUint32(ptr4+ offset, v, true);
      offset += 4;
    }
  } else {
    // TypedArray / ArrayBuffer-like, direct copy
    valData4 = new Uint8Array(val4.buffer || val4, val4.byteOffset, valLenBytes4);
    const out4 = new Uint8Array(memory0.buffer, ptr4, valLenBytes4);
    out4.set(valData4);
  }
  
  dataView(memory0).setUint32(arg2 + 4, len4, true);
  dataView(memory0).setUint32(arg2 + 0, ptr4, true);
  _debugLog('[iface="wasi:io/poll@0.2.9", function="poll"][Instruction::Return]', {
    funcName: 'poll',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline26.fnName = 'wasi:io/poll@0.2.9#poll';

const handleTable1 = [T_FLAG, 0];
handleTable1._createdReps = new Set();


const captureTable1= new Map();
let captureCnt1= 0;

HANDLE_TABLES[1] = handleTable1;

const _trampoline27 = function(arg0, arg1) {
  var handle1 = arg0;
  
  var rep2 = handleTable3[(handle1 << 1) + 1] & ~T_FLAG;
  var rsc0 = captureTable3.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(OutputStream.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.check-write"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'checkWrite',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'result-catch-handler',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  try {
    ret = { tag: 'ok', val: _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => rsc0.checkWrite(),
    })
  };
} catch (e) {
  ret = { tag: 'err', val: getErrorPayload(e) };
}

for (const entry of curResourceBorrows) {
  const rsc = entry.rsc ?? entry;
  if (entry.drop) {
    if (rsc[symbolRscHandle]) {
      entry.drop(rsc[symbolRscHandle]);
    }
  }
  rsc[symbolRscHandle] = undefined;
}
curResourceBorrows = [];
var variant5 = ret;
switch (variant5.tag) {
  case 'ok': {
    const e = variant5.val;
    dataView(memory0).setInt8(arg1 + 0, 0, true);
    dataView(memory0).setBigInt64(arg1 + 8, toUint64(e), true);
    
    break;
  }
  case 'err': {
    const e = variant5.val;
    dataView(memory0).setInt8(arg1 + 0, 1, true);
    var variant4 = e;
    switch (variant4.tag) {
      case 'last-operation-failed': {
        const e = variant4.val;
        dataView(memory0).setInt8(arg1 + 8, 0, true);
        
        if (!(e instanceof Error$1)) {
          throw new TypeError('Resource error: Not a valid \"Error\" resource.');
        }
        var handle3 = e[symbolRscHandle];
        if (!handle3) {
          const rep = e[symbolRscRep] || ++captureCnt1;
          captureTable1.set(rep, e);
          handle3 = rscTableCreateOwn(handleTable1, rep);
        }
        
        dataView(memory0).setInt32(arg1 + 12, handle3, true);
        break;
      }
      case 'closed': {
        dataView(memory0).setInt8(arg1 + 8, 1, true);
        break;
      }
      default: {
        throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant4.tag)}\` (received \`${variant4}\`) specified for \`StreamError\``);
      }
    }
    
    break;
  }
  default: {
    _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant5, valueType: typeof variant5});
    throw new TypeError('invalid variant specified for result');
  }
}
_debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.check-write"][Instruction::Return]', {
  funcName: '[method]output-stream.check-write',
  paramCount: 0,
  async: false,
  postReturn: false
});
task.resolve([ret]);
task.exit();
}
_trampoline27.fnName = 'wasi:io/streams@0.2.9#checkWrite';

const _trampoline28 = function(arg0, arg1, arg2, arg3) {
  var handle1 = arg0;
  
  var rep2 = handleTable3[(handle1 << 1) + 1] & ~T_FLAG;
  var rsc0 = captureTable3.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(OutputStream.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  var ptr3 = arg1;
  var len3 = arg2;
  var result3 = new Uint8Array(memory0.buffer.slice(ptr3, ptr3 + len3 * 1));
  _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.write"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'write',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'result-catch-handler',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  try {
    ret = { tag: 'ok', val: _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => rsc0.write(result3),
    })
  };
} catch (e) {
  ret = { tag: 'err', val: getErrorPayload(e) };
}

for (const entry of curResourceBorrows) {
  const rsc = entry.rsc ?? entry;
  if (entry.drop) {
    if (rsc[symbolRscHandle]) {
      entry.drop(rsc[symbolRscHandle]);
    }
  }
  rsc[symbolRscHandle] = undefined;
}
curResourceBorrows = [];
var variant6 = ret;
switch (variant6.tag) {
  case 'ok': {
    const e = variant6.val;
    dataView(memory0).setInt8(arg3 + 0, 0, true);
    
    break;
  }
  case 'err': {
    const e = variant6.val;
    dataView(memory0).setInt8(arg3 + 0, 1, true);
    var variant5 = e;
    switch (variant5.tag) {
      case 'last-operation-failed': {
        const e = variant5.val;
        dataView(memory0).setInt8(arg3 + 4, 0, true);
        
        if (!(e instanceof Error$1)) {
          throw new TypeError('Resource error: Not a valid \"Error\" resource.');
        }
        var handle4 = e[symbolRscHandle];
        if (!handle4) {
          const rep = e[symbolRscRep] || ++captureCnt1;
          captureTable1.set(rep, e);
          handle4 = rscTableCreateOwn(handleTable1, rep);
        }
        
        dataView(memory0).setInt32(arg3 + 8, handle4, true);
        break;
      }
      case 'closed': {
        dataView(memory0).setInt8(arg3 + 4, 1, true);
        break;
      }
      default: {
        throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant5.tag)}\` (received \`${variant5}\`) specified for \`StreamError\``);
      }
    }
    
    break;
  }
  default: {
    _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant6, valueType: typeof variant6});
    throw new TypeError('invalid variant specified for result');
  }
}
_debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.write"][Instruction::Return]', {
  funcName: '[method]output-stream.write',
  paramCount: 0,
  async: false,
  postReturn: false
});
task.resolve([ret]);
task.exit();
}
_trampoline28.fnName = 'wasi:io/streams@0.2.9#write';

const _trampoline29 = function(arg0, arg1) {
  var handle1 = arg0;
  
  var rep2 = handleTable3[(handle1 << 1) + 1] & ~T_FLAG;
  var rsc0 = captureTable3.get(rep2);
  if (!rsc0) {
    rsc0 = Object.create(OutputStream.prototype);
    Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
    Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
  }
  
  curResourceBorrows.push(rsc0);
  _debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.blocking-flush"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'blockingFlush',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'result-catch-handler',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  try {
    ret = { tag: 'ok', val: _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => rsc0.blockingFlush(),
    })
  };
} catch (e) {
  ret = { tag: 'err', val: getErrorPayload(e) };
}

for (const entry of curResourceBorrows) {
  const rsc = entry.rsc ?? entry;
  if (entry.drop) {
    if (rsc[symbolRscHandle]) {
      entry.drop(rsc[symbolRscHandle]);
    }
  }
  rsc[symbolRscHandle] = undefined;
}
curResourceBorrows = [];
var variant5 = ret;
switch (variant5.tag) {
  case 'ok': {
    const e = variant5.val;
    dataView(memory0).setInt8(arg1 + 0, 0, true);
    
    break;
  }
  case 'err': {
    const e = variant5.val;
    dataView(memory0).setInt8(arg1 + 0, 1, true);
    var variant4 = e;
    switch (variant4.tag) {
      case 'last-operation-failed': {
        const e = variant4.val;
        dataView(memory0).setInt8(arg1 + 4, 0, true);
        
        if (!(e instanceof Error$1)) {
          throw new TypeError('Resource error: Not a valid \"Error\" resource.');
        }
        var handle3 = e[symbolRscHandle];
        if (!handle3) {
          const rep = e[symbolRscRep] || ++captureCnt1;
          captureTable1.set(rep, e);
          handle3 = rscTableCreateOwn(handleTable1, rep);
        }
        
        dataView(memory0).setInt32(arg1 + 8, handle3, true);
        break;
      }
      case 'closed': {
        dataView(memory0).setInt8(arg1 + 4, 1, true);
        break;
      }
      default: {
        throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant4.tag)}\` (received \`${variant4}\`) specified for \`StreamError\``);
      }
    }
    
    break;
  }
  default: {
    _debugLog("ERROR: invalid value (expected result as object with 'tag' member)", { value: variant5, valueType: typeof variant5});
    throw new TypeError('invalid variant specified for result');
  }
}
_debugLog('[iface="wasi:io/streams@0.2.9", function="[method]output-stream.blocking-flush"][Instruction::Return]', {
  funcName: '[method]output-stream.blocking-flush',
  paramCount: 0,
  async: false,
  postReturn: false
});
task.resolve([ret]);
task.exit();
}
_trampoline29.fnName = 'wasi:io/streams@0.2.9#blockingFlush';

const _trampoline30 = function(arg0) {
  _debugLog('[iface="wasi:cli/environment@0.2.9", function="get-environment"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getEnvironment',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getEnvironment(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var vec3 = ret;
  var len3 = vec3.length;
  var result3 = realloc0(0, 0, 4, len3 * 16);
  for (let i = 0; i < vec3.length; i++) {
    const e = vec3[i];
    const base = result3 + i * 16;var [tuple0_0, tuple0_1] = e;
    
    var encodeRes = _utf8AllocateAndEncode(tuple0_0, realloc0, memory0);
    var ptr1= encodeRes.ptr;
    var len1 = encodeRes.len;
    
    dataView(memory0).setUint32(base + 4, len1, true);
    dataView(memory0).setUint32(base + 0, ptr1, true);
    
    var encodeRes = _utf8AllocateAndEncode(tuple0_1, realloc0, memory0);
    var ptr2= encodeRes.ptr;
    var len2 = encodeRes.len;
    
    dataView(memory0).setUint32(base + 12, len2, true);
    dataView(memory0).setUint32(base + 8, ptr2, true);
  }
  dataView(memory0).setUint32(arg0 + 4, len3, true);
  dataView(memory0).setUint32(arg0 + 0, result3, true);
  _debugLog('[iface="wasi:cli/environment@0.2.9", function="get-environment"][Instruction::Return]', {
    funcName: 'get-environment',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline30.fnName = 'wasi:cli/environment@0.2.9#getEnvironment';

const handleTable4 = [T_FLAG, 0];
handleTable4._createdReps = new Set();


const captureTable4= new Map();
let captureCnt4= 0;

HANDLE_TABLES[4] = handleTable4;

const _trampoline31 = function(arg0) {
  _debugLog('[iface="wasi:cli/terminal-stdin@0.2.9", function="get-terminal-stdin"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTerminalStdin',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getTerminalStdin(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(arg0 + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(arg0 + 0, 1, true);
    
    if (!(e instanceof TerminalInput)) {
      throw new TypeError('Resource error: Not a valid \"TerminalInput\" resource.');
    }
    var handle0 = e[symbolRscHandle];
    if (!handle0) {
      const rep = e[symbolRscRep] || ++captureCnt4;
      captureTable4.set(rep, e);
      handle0 = rscTableCreateOwn(handleTable4, rep);
    }
    
    dataView(memory0).setInt32(arg0 + 4, handle0, true);
  }
  _debugLog('[iface="wasi:cli/terminal-stdin@0.2.9", function="get-terminal-stdin"][Instruction::Return]', {
    funcName: 'get-terminal-stdin',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline31.fnName = 'wasi:cli/terminal-stdin@0.2.9#getTerminalStdin';

const handleTable5 = [T_FLAG, 0];
handleTable5._createdReps = new Set();


const captureTable5= new Map();
let captureCnt5= 0;

HANDLE_TABLES[5] = handleTable5;

const _trampoline32 = function(arg0) {
  _debugLog('[iface="wasi:cli/terminal-stdout@0.2.9", function="get-terminal-stdout"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTerminalStdout',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getTerminalStdout(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(arg0 + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(arg0 + 0, 1, true);
    
    if (!(e instanceof TerminalOutput)) {
      throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
    }
    var handle0 = e[symbolRscHandle];
    if (!handle0) {
      const rep = e[symbolRscRep] || ++captureCnt5;
      captureTable5.set(rep, e);
      handle0 = rscTableCreateOwn(handleTable5, rep);
    }
    
    dataView(memory0).setInt32(arg0 + 4, handle0, true);
  }
  _debugLog('[iface="wasi:cli/terminal-stdout@0.2.9", function="get-terminal-stdout"][Instruction::Return]', {
    funcName: 'get-terminal-stdout',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline32.fnName = 'wasi:cli/terminal-stdout@0.2.9#getTerminalStdout';

const _trampoline33 = function(arg0) {
  _debugLog('[iface="wasi:cli/terminal-stderr@0.2.9", function="get-terminal-stderr"] [Instruction::CallInterface] (sync, @ enter)');
  const hostProvided = true;
  
  let parentTask;
  let task;
  let subtask;
  
  const createTask = () => {
    const results = createNewCurrentTask({
      componentIdx: -1,
      isAsync: false,
      entryFnName: 'getTerminalStderr',
      getCallbackFn: () => null,
      callbackFnName: null,
      errHandling: 'none',
      callingWasmExport: false,
    });
    task = results[0];
  };
  
  taskCreation: {
    parentTask = getCurrentTask(
    0,
    _getGlobalCurrentTaskMeta(0)?.taskID,
    )?.task;
    
    if (!parentTask) {
      createTask();
      break taskCreation;
    }
    
    createTask();
    
    if (hostProvided) {
      subtask = parentTask.getLatestSubtask();
      if (!subtask) {
        throw new Error(`Missing subtask (in parent task [${parentTask.id()}]) for host import, has the import been lowered? (ensure asyncImports are set properly)`);
      }
      task.setParentSubtask(subtask);
    }
  }
  
  const started = task.enterSync();
  
  let ret;
  
  try {
    ret = _withGlobalCurrentTaskMeta({
      componentIdx: task.componentIdx(),
      taskID: task.id(),
      fn: () => getTerminalStderr(),
    })
    ;
  } catch (err) {
    
    _debugLog('[Instruction::CallInterface] error during sync call', {
      taskID: task.id(),
      subtaskID: task.getParentSubtask()?.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    throw err;
    
  }
  
  var variant1 = ret;
  if (variant1 === null || variant1=== undefined) {
    dataView(memory0).setInt8(arg0 + 0, 0, true);
  } else {
    const e = variant1;
    dataView(memory0).setInt8(arg0 + 0, 1, true);
    
    if (!(e instanceof TerminalOutput)) {
      throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
    }
    var handle0 = e[symbolRscHandle];
    if (!handle0) {
      const rep = e[symbolRscRep] || ++captureCnt5;
      captureTable5.set(rep, e);
      handle0 = rscTableCreateOwn(handleTable5, rep);
    }
    
    dataView(memory0).setInt32(arg0 + 4, handle0, true);
  }
  _debugLog('[iface="wasi:cli/terminal-stderr@0.2.9", function="get-terminal-stderr"][Instruction::Return]', {
    funcName: 'get-terminal-stderr',
    paramCount: 0,
    async: false,
    postReturn: false
  });
  task.resolve([ret]);
  task.exit();
}
_trampoline33.fnName = 'wasi:cli/terminal-stderr@0.2.9#getTerminalStderr';
let exports2;
let callback_0;
let reactor100Poll;

async function poll$1(arg0, arg1) {
  var vec81 = arg0;
  var len81 = vec81.length;
  var result81 = await realloc0Async(0, 0, 8, len81 * 64);
  for (let i = 0; i < vec81.length; i++) {
    const e = vec81[i];
    const base = result81 + i * 64;var variant80 = e;
    switch (variant80.tag) {
      case 'instance-open': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 0, true);
        var {instance: v0_0, appId: v0_1, actor: v0_2, config: v0_3, assets: v0_4, capabilities: v0_5, quotas: v0_6 } = e;
        dataView(memory0).setInt32(base + 8, toUint32(v0_0), true);
        
        var encodeRes = await _utf8AllocateAndEncodeAsync(v0_1, realloc0Async, memory0);
        var ptr1= encodeRes.ptr;
        var len1 = encodeRes.len;
        
        dataView(memory0).setUint32(base + 16, len1, true);
        dataView(memory0).setUint32(base + 12, ptr1, true);
        
        var encodeRes = await _utf8AllocateAndEncodeAsync(v0_2, realloc0Async, memory0);
        var ptr2= encodeRes.ptr;
        var len2 = encodeRes.len;
        
        dataView(memory0).setUint32(base + 24, len2, true);
        dataView(memory0).setUint32(base + 20, ptr2, true);
        var val3 = v0_3;
        var len3 = Array.isArray(val3) ? val3.length : val3.byteLength;
        var ptr3 = await realloc0Async(0, 0, 1, len3 * 1);
        
        let valData3;
        const valLenBytes3 = len3 * 1;
        if (Array.isArray(val3)) {
          // Regular array likely containing numbers, write values to memory
          let offset = 0;
          const dv3 = new DataView(memory0.buffer);
          for (const v of val3) {
            _requireValidNumericPrimitive.bind(null, 'u8')(v);
            dv3.setUint8(ptr3+ offset, v, true);
            offset += 1;
          }
        } else {
          // TypedArray / ArrayBuffer-like, direct copy
          valData3 = new Uint8Array(val3.buffer || val3, val3.byteOffset, valLenBytes3);
          const out3 = new Uint8Array(memory0.buffer, ptr3, valLenBytes3);
          out3.set(valData3);
        }
        
        dataView(memory0).setUint32(base + 32, len3, true);
        dataView(memory0).setUint32(base + 28, ptr3, true);
        var vec7 = v0_4;
        var len7 = vec7.length;
        var result7 = await realloc0Async(0, 0, 4, len7 * 16);
        for (let i = 0; i < vec7.length; i++) {
          const e = vec7[i];
          const base = result7 + i * 16;var [tuple4_0, tuple4_1] = e;
          
          var encodeRes = await _utf8AllocateAndEncodeAsync(tuple4_0, realloc0Async, memory0);
          var ptr5= encodeRes.ptr;
          var len5 = encodeRes.len;
          
          dataView(memory0).setUint32(base + 4, len5, true);
          dataView(memory0).setUint32(base + 0, ptr5, true);
          var val6 = tuple4_1;
          var len6 = Array.isArray(val6) ? val6.length : val6.byteLength;
          var ptr6 = await realloc0Async(0, 0, 1, len6 * 1);
          
          let valData6;
          const valLenBytes6 = len6 * 1;
          if (Array.isArray(val6)) {
            // Regular array likely containing numbers, write values to memory
            let offset = 0;
            const dv6 = new DataView(memory0.buffer);
            for (const v of val6) {
              _requireValidNumericPrimitive.bind(null, 'u8')(v);
              dv6.setUint8(ptr6+ offset, v, true);
              offset += 1;
            }
          } else {
            // TypedArray / ArrayBuffer-like, direct copy
            valData6 = new Uint8Array(val6.buffer || val6, val6.byteOffset, valLenBytes6);
            const out6 = new Uint8Array(memory0.buffer, ptr6, valLenBytes6);
            out6.set(valData6);
          }
          
          dataView(memory0).setUint32(base + 12, len6, true);
          dataView(memory0).setUint32(base + 8, ptr6, true);
        }
        dataView(memory0).setUint32(base + 40, len7, true);
        dataView(memory0).setUint32(base + 36, result7, true);
        var vec13 = v0_5;
        var len13 = vec13.length;
        var result13 = await realloc0Async(0, 0, 8, len13 * 40);
        for (let i = 0; i < vec13.length; i++) {
          const e = vec13[i];
          const base = result13 + i * 40;var {token: v8_0, scope: v8_1, expiresMs: v8_2 } = e;
          var {id: v9_0, token: v9_1 } = v8_0;
          
          var encodeRes = await _utf8AllocateAndEncodeAsync(v9_0, realloc0Async, memory0);
          var ptr10= encodeRes.ptr;
          var len10 = encodeRes.len;
          
          dataView(memory0).setUint32(base + 4, len10, true);
          dataView(memory0).setUint32(base + 0, ptr10, true);
          dataView(memory0).setBigInt64(base + 8, toUint64(v9_1), true);
          
          var encodeRes = await _utf8AllocateAndEncodeAsync(v8_1, realloc0Async, memory0);
          var ptr11= encodeRes.ptr;
          var len11 = encodeRes.len;
          
          dataView(memory0).setUint32(base + 20, len11, true);
          dataView(memory0).setUint32(base + 16, ptr11, true);
          var variant12 = v8_2;
          if (variant12 === null || variant12=== undefined) {
            dataView(memory0).setInt8(base + 24, 0, true);
          } else {
            const e = variant12;
            dataView(memory0).setInt8(base + 24, 1, true);
            dataView(memory0).setBigInt64(base + 32, toInt64(e), true);
          }
        }
        dataView(memory0).setUint32(base + 48, len13, true);
        dataView(memory0).setUint32(base + 44, result13, true);
        var val14 = v0_6;
        var len14 = Array.isArray(val14) ? val14.length : val14.byteLength;
        var ptr14 = await realloc0Async(0, 0, 1, len14 * 1);
        
        let valData14;
        const valLenBytes14 = len14 * 1;
        if (Array.isArray(val14)) {
          // Regular array likely containing numbers, write values to memory
          let offset = 0;
          const dv14 = new DataView(memory0.buffer);
          for (const v of val14) {
            _requireValidNumericPrimitive.bind(null, 'u8')(v);
            dv14.setUint8(ptr14+ offset, v, true);
            offset += 1;
          }
        } else {
          // TypedArray / ArrayBuffer-like, direct copy
          valData14 = new Uint8Array(val14.buffer || val14, val14.byteOffset, valLenBytes14);
          const out14 = new Uint8Array(memory0.buffer, ptr14, valLenBytes14);
          out14.set(valData14);
        }
        
        dataView(memory0).setUint32(base + 56, len14, true);
        dataView(memory0).setUint32(base + 52, ptr14, true);
        break;
      }
      case 'instance-close': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 1, true);
        var {instance: v15_0 } = e;
        dataView(memory0).setInt32(base + 8, toUint32(v15_0), true);
        break;
      }
      case 'activate': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 2, true);
        var {instance: v16_0, reason: v16_1 } = e;
        dataView(memory0).setInt32(base + 8, toUint32(v16_0), true);
        var variant22 = v16_1;
        switch (variant22.tag) {
          case 'on-command': {
            const e = variant22.val;
            dataView(memory0).setInt8(base + 12, 0, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr17= encodeRes.ptr;
            var len17 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 20, len17, true);
            dataView(memory0).setUint32(base + 16, ptr17, true);
            break;
          }
          case 'on-view-visible': {
            const e = variant22.val;
            dataView(memory0).setInt8(base + 12, 1, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr18= encodeRes.ptr;
            var len18 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 20, len18, true);
            dataView(memory0).setUint32(base + 16, ptr18, true);
            break;
          }
          case 'on-file-type': {
            const e = variant22.val;
            dataView(memory0).setInt8(base + 12, 2, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr19= encodeRes.ptr;
            var len19 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 20, len19, true);
            dataView(memory0).setUint32(base + 16, ptr19, true);
            break;
          }
          case 'on-artifact-kind': {
            const e = variant22.val;
            dataView(memory0).setInt8(base + 12, 3, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr20= encodeRes.ptr;
            var len20 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 20, len20, true);
            dataView(memory0).setUint32(base + 16, ptr20, true);
            break;
          }
          case 'on-extension-request': {
            const e = variant22.val;
            dataView(memory0).setInt8(base + 12, 4, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr21= encodeRes.ptr;
            var len21 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 20, len21, true);
            dataView(memory0).setUint32(base + 16, ptr21, true);
            break;
          }
          case 'on-startup-finished': {
            dataView(memory0).setInt8(base + 12, 5, true);
            break;
          }
          default: {
            throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant22.tag)}\` (received \`${variant22}\`) specified for \`ActivationEvent\``);
          }
        }
        break;
      }
      case 'suspend-request': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 3, true);
        var {instance: v23_0 } = e;
        dataView(memory0).setInt32(base + 8, toUint32(v23_0), true);
        break;
      }
      case 'capability-changed': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 4, true);
        var {instance: v24_0, change: v24_1 } = e;
        dataView(memory0).setInt32(base + 8, toUint32(v24_0), true);
        var variant36 = v24_1;
        switch (variant36.tag) {
          case 'granted': {
            const e = variant36.val;
            dataView(memory0).setInt8(base + 16, 0, true);
            var {token: v25_0, scope: v25_1, expiresMs: v25_2 } = e;
            var {id: v26_0, token: v26_1 } = v25_0;
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(v26_0, realloc0Async, memory0);
            var ptr27= encodeRes.ptr;
            var len27 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 28, len27, true);
            dataView(memory0).setUint32(base + 24, ptr27, true);
            dataView(memory0).setBigInt64(base + 32, toUint64(v26_1), true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(v25_1, realloc0Async, memory0);
            var ptr28= encodeRes.ptr;
            var len28 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 44, len28, true);
            dataView(memory0).setUint32(base + 40, ptr28, true);
            var variant29 = v25_2;
            if (variant29 === null || variant29=== undefined) {
              dataView(memory0).setInt8(base + 48, 0, true);
            } else {
              const e = variant29;
              dataView(memory0).setInt8(base + 48, 1, true);
              dataView(memory0).setBigInt64(base + 56, toInt64(e), true);
            }
            break;
          }
          case 'revoked': {
            const e = variant36.val;
            dataView(memory0).setInt8(base + 16, 1, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr30= encodeRes.ptr;
            var len30 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 28, len30, true);
            dataView(memory0).setUint32(base + 24, ptr30, true);
            break;
          }
          case 'narrowed': {
            const e = variant36.val;
            dataView(memory0).setInt8(base + 16, 2, true);
            var {token: v31_0, scope: v31_1, expiresMs: v31_2 } = e;
            var {id: v32_0, token: v32_1 } = v31_0;
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(v32_0, realloc0Async, memory0);
            var ptr33= encodeRes.ptr;
            var len33 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 28, len33, true);
            dataView(memory0).setUint32(base + 24, ptr33, true);
            dataView(memory0).setBigInt64(base + 32, toUint64(v32_1), true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(v31_1, realloc0Async, memory0);
            var ptr34= encodeRes.ptr;
            var len34 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 44, len34, true);
            dataView(memory0).setUint32(base + 40, ptr34, true);
            var variant35 = v31_2;
            if (variant35 === null || variant35=== undefined) {
              dataView(memory0).setInt8(base + 48, 0, true);
            } else {
              const e = variant35;
              dataView(memory0).setInt8(base + 48, 1, true);
              dataView(memory0).setBigInt64(base + 56, toInt64(e), true);
            }
            break;
          }
          default: {
            throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant36.tag)}\` (received \`${variant36}\`) specified for \`CapabilityChange\``);
          }
        }
        break;
      }
      case 'quota-changed': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 5, true);
        var {instance: v37_0, quotas: v37_1 } = e;
        dataView(memory0).setInt32(base + 8, toUint32(v37_0), true);
        var val38 = v37_1;
        var len38 = Array.isArray(val38) ? val38.length : val38.byteLength;
        var ptr38 = await realloc0Async(0, 0, 1, len38 * 1);
        
        let valData38;
        const valLenBytes38 = len38 * 1;
        if (Array.isArray(val38)) {
          // Regular array likely containing numbers, write values to memory
          let offset = 0;
          const dv38 = new DataView(memory0.buffer);
          for (const v of val38) {
            _requireValidNumericPrimitive.bind(null, 'u8')(v);
            dv38.setUint8(ptr38+ offset, v, true);
            offset += 1;
          }
        } else {
          // TypedArray / ArrayBuffer-like, direct copy
          valData38 = new Uint8Array(val38.buffer || val38, val38.byteOffset, valLenBytes38);
          const out38 = new Uint8Array(memory0.buffer, ptr38, valLenBytes38);
          out38.set(valData38);
        }
        
        dataView(memory0).setUint32(base + 16, len38, true);
        dataView(memory0).setUint32(base + 12, ptr38, true);
        break;
      }
      case 'app-command': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 6, true);
        var {instance: v39_0, seq: v39_1, command: v39_2 } = e;
        dataView(memory0).setInt32(base + 8, toUint32(v39_0), true);
        dataView(memory0).setBigInt64(base + 16, toUint64(v39_1), true);
        var val40 = v39_2;
        var len40 = Array.isArray(val40) ? val40.length : val40.byteLength;
        var ptr40 = await realloc0Async(0, 0, 1, len40 * 1);
        
        let valData40;
        const valLenBytes40 = len40 * 1;
        if (Array.isArray(val40)) {
          // Regular array likely containing numbers, write values to memory
          let offset = 0;
          const dv40 = new DataView(memory0.buffer);
          for (const v of val40) {
            _requireValidNumericPrimitive.bind(null, 'u8')(v);
            dv40.setUint8(ptr40+ offset, v, true);
            offset += 1;
          }
        } else {
          // TypedArray / ArrayBuffer-like, direct copy
          valData40 = new Uint8Array(val40.buffer || val40, val40.byteOffset, valLenBytes40);
          const out40 = new Uint8Array(memory0.buffer, ptr40, valLenBytes40);
          out40.set(valData40);
        }
        
        dataView(memory0).setUint32(base + 28, len40, true);
        dataView(memory0).setUint32(base + 24, ptr40, true);
        break;
      }
      case 'surface-visible': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 7, true);
        var {surface: v41_0 } = e;
        var {instance: v42_0, surface: v42_1 } = v41_0;
        dataView(memory0).setInt32(base + 8, toUint32(v42_0), true);
        dataView(memory0).setInt32(base + 12, toUint32(v42_1), true);
        break;
      }
      case 'surface-hidden': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 8, true);
        var {surface: v43_0 } = e;
        var {instance: v44_0, surface: v44_1 } = v43_0;
        dataView(memory0).setInt32(base + 8, toUint32(v44_0), true);
        dataView(memory0).setInt32(base + 12, toUint32(v44_1), true);
        break;
      }
      case 'surface-resized': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 9, true);
        var {surface: v45_0, width: v45_1, height: v45_2 } = e;
        var {instance: v46_0, surface: v46_1 } = v45_0;
        dataView(memory0).setInt32(base + 8, toUint32(v46_0), true);
        dataView(memory0).setInt32(base + 12, toUint32(v46_1), true);
        dataView(memory0).setInt32(base + 16, toUint32(v45_1), true);
        dataView(memory0).setInt32(base + 20, toUint32(v45_2), true);
        break;
      }
      case 'patch-ack': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 10, true);
        var {surface: v47_0, revision: v47_1 } = e;
        var {instance: v48_0, surface: v48_1 } = v47_0;
        dataView(memory0).setInt32(base + 8, toUint32(v48_0), true);
        dataView(memory0).setInt32(base + 12, toUint32(v48_1), true);
        dataView(memory0).setBigInt64(base + 16, toUint64(v47_1), true);
        break;
      }
      case 'patch-rejected': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 11, true);
        var {surface: v49_0, revision: v49_1, reason: v49_2 } = e;
        var {instance: v50_0, surface: v50_1 } = v49_0;
        dataView(memory0).setInt32(base + 8, toUint32(v50_0), true);
        dataView(memory0).setInt32(base + 12, toUint32(v50_1), true);
        dataView(memory0).setBigInt64(base + 16, toUint64(v49_1), true);
        
        var encodeRes = await _utf8AllocateAndEncodeAsync(v49_2, realloc0Async, memory0);
        var ptr51= encodeRes.ptr;
        var len51 = encodeRes.len;
        
        dataView(memory0).setUint32(base + 28, len51, true);
        dataView(memory0).setUint32(base + 24, ptr51, true);
        break;
      }
      case 'completed': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 12, true);
        var {req: v52_0, outcome: v52_1 } = e;
        dataView(memory0).setBigInt64(base + 8, toUint64(v52_0), true);
        var variant55 = v52_1;
        switch (variant55.tag) {
          case 'ok': {
            const e = variant55.val;
            dataView(memory0).setInt8(base + 16, 0, true);
            var val53 = e;
            var len53 = Array.isArray(val53) ? val53.length : val53.byteLength;
            var ptr53 = await realloc0Async(0, 0, 1, len53 * 1);
            
            let valData53;
            const valLenBytes53 = len53 * 1;
            if (Array.isArray(val53)) {
              // Regular array likely containing numbers, write values to memory
              let offset = 0;
              const dv53 = new DataView(memory0.buffer);
              for (const v of val53) {
                _requireValidNumericPrimitive.bind(null, 'u8')(v);
                dv53.setUint8(ptr53+ offset, v, true);
                offset += 1;
              }
            } else {
              // TypedArray / ArrayBuffer-like, direct copy
              valData53 = new Uint8Array(val53.buffer || val53, val53.byteOffset, valLenBytes53);
              const out53 = new Uint8Array(memory0.buffer, ptr53, valLenBytes53);
              out53.set(valData53);
            }
            
            dataView(memory0).setUint32(base + 24, len53, true);
            dataView(memory0).setUint32(base + 20, ptr53, true);
            break;
          }
          case 'fault': {
            const e = variant55.val;
            dataView(memory0).setInt8(base + 16, 1, true);
            var val54 = e;
            var len54 = Array.isArray(val54) ? val54.length : val54.byteLength;
            var ptr54 = await realloc0Async(0, 0, 1, len54 * 1);
            
            let valData54;
            const valLenBytes54 = len54 * 1;
            if (Array.isArray(val54)) {
              // Regular array likely containing numbers, write values to memory
              let offset = 0;
              const dv54 = new DataView(memory0.buffer);
              for (const v of val54) {
                _requireValidNumericPrimitive.bind(null, 'u8')(v);
                dv54.setUint8(ptr54+ offset, v, true);
                offset += 1;
              }
            } else {
              // TypedArray / ArrayBuffer-like, direct copy
              valData54 = new Uint8Array(val54.buffer || val54, val54.byteOffset, valLenBytes54);
              const out54 = new Uint8Array(memory0.buffer, ptr54, valLenBytes54);
              out54.set(valData54);
            }
            
            dataView(memory0).setUint32(base + 24, len54, true);
            dataView(memory0).setUint32(base + 20, ptr54, true);
            break;
          }
          default: {
            throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant55.tag)}\` (received \`${variant55}\`) specified for \`CompletionResult\``);
          }
        }
        break;
      }
      case 'http-chunk': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 13, true);
        var {req: v56_0, params: v56_1 } = e;
        dataView(memory0).setBigInt64(base + 8, toUint64(v56_0), true);
        var {bytes: v57_0, done: v57_1 } = v56_1;
        var val58 = v57_0;
        var len58 = Array.isArray(val58) ? val58.length : val58.byteLength;
        var ptr58 = await realloc0Async(0, 0, 1, len58 * 1);
        
        let valData58;
        const valLenBytes58 = len58 * 1;
        if (Array.isArray(val58)) {
          // Regular array likely containing numbers, write values to memory
          let offset = 0;
          const dv58 = new DataView(memory0.buffer);
          for (const v of val58) {
            _requireValidNumericPrimitive.bind(null, 'u8')(v);
            dv58.setUint8(ptr58+ offset, v, true);
            offset += 1;
          }
        } else {
          // TypedArray / ArrayBuffer-like, direct copy
          valData58 = new Uint8Array(val58.buffer || val58, val58.byteOffset, valLenBytes58);
          const out58 = new Uint8Array(memory0.buffer, ptr58, valLenBytes58);
          out58.set(valData58);
        }
        
        dataView(memory0).setUint32(base + 20, len58, true);
        dataView(memory0).setUint32(base + 16, ptr58, true);
        dataView(memory0).setInt8(base + 24, v57_1 ? 1 : 0, true);
        break;
      }
      case 'job-progress': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 14, true);
        var {job: v59_0, progress: v59_1 } = e;
        dataView(memory0).setBigInt64(base + 8, toUint64(v59_0), true);
        var val60 = v59_1;
        var len60 = Array.isArray(val60) ? val60.length : val60.byteLength;
        var ptr60 = await realloc0Async(0, 0, 1, len60 * 1);
        
        let valData60;
        const valLenBytes60 = len60 * 1;
        if (Array.isArray(val60)) {
          // Regular array likely containing numbers, write values to memory
          let offset = 0;
          const dv60 = new DataView(memory0.buffer);
          for (const v of val60) {
            _requireValidNumericPrimitive.bind(null, 'u8')(v);
            dv60.setUint8(ptr60+ offset, v, true);
            offset += 1;
          }
        } else {
          // TypedArray / ArrayBuffer-like, direct copy
          valData60 = new Uint8Array(val60.buffer || val60, val60.byteOffset, valLenBytes60);
          const out60 = new Uint8Array(memory0.buffer, ptr60, valLenBytes60);
          out60.set(valData60);
        }
        
        dataView(memory0).setUint32(base + 20, len60, true);
        dataView(memory0).setUint32(base + 16, ptr60, true);
        break;
      }
      case 'job-completed': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 15, true);
        var {job: v61_0, outcome: v61_1 } = e;
        dataView(memory0).setBigInt64(base + 8, toUint64(v61_0), true);
        var variant64 = v61_1;
        switch (variant64.tag) {
          case 'ok': {
            const e = variant64.val;
            dataView(memory0).setInt8(base + 16, 0, true);
            var val62 = e;
            var len62 = Array.isArray(val62) ? val62.length : val62.byteLength;
            var ptr62 = await realloc0Async(0, 0, 1, len62 * 1);
            
            let valData62;
            const valLenBytes62 = len62 * 1;
            if (Array.isArray(val62)) {
              // Regular array likely containing numbers, write values to memory
              let offset = 0;
              const dv62 = new DataView(memory0.buffer);
              for (const v of val62) {
                _requireValidNumericPrimitive.bind(null, 'u8')(v);
                dv62.setUint8(ptr62+ offset, v, true);
                offset += 1;
              }
            } else {
              // TypedArray / ArrayBuffer-like, direct copy
              valData62 = new Uint8Array(val62.buffer || val62, val62.byteOffset, valLenBytes62);
              const out62 = new Uint8Array(memory0.buffer, ptr62, valLenBytes62);
              out62.set(valData62);
            }
            
            dataView(memory0).setUint32(base + 24, len62, true);
            dataView(memory0).setUint32(base + 20, ptr62, true);
            break;
          }
          case 'fault': {
            const e = variant64.val;
            dataView(memory0).setInt8(base + 16, 1, true);
            var val63 = e;
            var len63 = Array.isArray(val63) ? val63.length : val63.byteLength;
            var ptr63 = await realloc0Async(0, 0, 1, len63 * 1);
            
            let valData63;
            const valLenBytes63 = len63 * 1;
            if (Array.isArray(val63)) {
              // Regular array likely containing numbers, write values to memory
              let offset = 0;
              const dv63 = new DataView(memory0.buffer);
              for (const v of val63) {
                _requireValidNumericPrimitive.bind(null, 'u8')(v);
                dv63.setUint8(ptr63+ offset, v, true);
                offset += 1;
              }
            } else {
              // TypedArray / ArrayBuffer-like, direct copy
              valData63 = new Uint8Array(val63.buffer || val63, val63.byteOffset, valLenBytes63);
              const out63 = new Uint8Array(memory0.buffer, ptr63, valLenBytes63);
              out63.set(valData63);
            }
            
            dataView(memory0).setUint32(base + 24, len63, true);
            dataView(memory0).setUint32(base + 20, ptr63, true);
            break;
          }
          default: {
            throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant64.tag)}\` (received \`${variant64}\`) specified for \`CompletionResult\``);
          }
        }
        break;
      }
      case 'message': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 16, true);
        var {source: v65_0, payload: v65_1 } = e;
        var variant69 = v65_0;
        switch (variant69.tag) {
          case 'shell': {
            const e = variant69.val;
            dataView(memory0).setInt8(base + 8, 0, true);
            dataView(memory0).setInt32(base + 12, toUint32(e), true);
            break;
          }
          case 'backbone': {
            const e = variant69.val;
            dataView(memory0).setInt8(base + 8, 1, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr66= encodeRes.ptr;
            var len66 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 16, len66, true);
            dataView(memory0).setUint32(base + 12, ptr66, true);
            break;
          }
          case 'plugin-instance': {
            const e = variant69.val;
            dataView(memory0).setInt8(base + 8, 2, true);
            dataView(memory0).setInt32(base + 12, toUint32(e), true);
            break;
          }
          case 'extension': {
            const e = variant69.val;
            dataView(memory0).setInt8(base + 8, 3, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr67= encodeRes.ptr;
            var len67 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 16, len67, true);
            dataView(memory0).setUint32(base + 12, ptr67, true);
            break;
          }
          case 'topic': {
            const e = variant69.val;
            dataView(memory0).setInt8(base + 8, 4, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr68= encodeRes.ptr;
            var len68 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 16, len68, true);
            dataView(memory0).setUint32(base + 12, ptr68, true);
            break;
          }
          default: {
            throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant69.tag)}\` (received \`${variant69}\`) specified for \`MessageEndpoint\``);
          }
        }
        var val70 = v65_1;
        var len70 = Array.isArray(val70) ? val70.length : val70.byteLength;
        var ptr70 = await realloc0Async(0, 0, 1, len70 * 1);
        
        let valData70;
        const valLenBytes70 = len70 * 1;
        if (Array.isArray(val70)) {
          // Regular array likely containing numbers, write values to memory
          let offset = 0;
          const dv70 = new DataView(memory0.buffer);
          for (const v of val70) {
            _requireValidNumericPrimitive.bind(null, 'u8')(v);
            dv70.setUint8(ptr70+ offset, v, true);
            offset += 1;
          }
        } else {
          // TypedArray / ArrayBuffer-like, direct copy
          valData70 = new Uint8Array(val70.buffer || val70, val70.byteOffset, valLenBytes70);
          const out70 = new Uint8Array(memory0.buffer, ptr70, valLenBytes70);
          out70.set(valData70);
        }
        
        dataView(memory0).setUint32(base + 24, len70, true);
        dataView(memory0).setUint32(base + 20, ptr70, true);
        break;
      }
      case 'timer': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 17, true);
        var {id: v71_0 } = e;
        dataView(memory0).setBigInt64(base + 8, toUint64(v71_0), true);
        break;
      }
      case 'wake': {
        dataView(memory0).setInt8(base + 0, 18, true);
        break;
      }
      case 'request': {
        const e = variant80.val;
        dataView(memory0).setInt8(base + 0, 19, true);
        var {req: v72_0, params: v72_1 } = e;
        dataView(memory0).setBigInt64(base + 8, toUint64(v72_0), true);
        var {origin: v73_0, capability: v73_1, payload: v73_2 } = v72_1;
        var variant77 = v73_0;
        switch (variant77.tag) {
          case 'shell': {
            const e = variant77.val;
            dataView(memory0).setInt8(base + 16, 0, true);
            dataView(memory0).setInt32(base + 20, toUint32(e), true);
            break;
          }
          case 'backbone': {
            const e = variant77.val;
            dataView(memory0).setInt8(base + 16, 1, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr74= encodeRes.ptr;
            var len74 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 24, len74, true);
            dataView(memory0).setUint32(base + 20, ptr74, true);
            break;
          }
          case 'plugin-instance': {
            const e = variant77.val;
            dataView(memory0).setInt8(base + 16, 2, true);
            dataView(memory0).setInt32(base + 20, toUint32(e), true);
            break;
          }
          case 'extension': {
            const e = variant77.val;
            dataView(memory0).setInt8(base + 16, 3, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr75= encodeRes.ptr;
            var len75 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 24, len75, true);
            dataView(memory0).setUint32(base + 20, ptr75, true);
            break;
          }
          case 'topic': {
            const e = variant77.val;
            dataView(memory0).setInt8(base + 16, 4, true);
            
            var encodeRes = await _utf8AllocateAndEncodeAsync(e, realloc0Async, memory0);
            var ptr76= encodeRes.ptr;
            var len76 = encodeRes.len;
            
            dataView(memory0).setUint32(base + 24, len76, true);
            dataView(memory0).setUint32(base + 20, ptr76, true);
            break;
          }
          default: {
            throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant77.tag)}\` (received \`${variant77}\`) specified for \`MessageEndpoint\``);
          }
        }
        
        var encodeRes = await _utf8AllocateAndEncodeAsync(v73_1, realloc0Async, memory0);
        var ptr78= encodeRes.ptr;
        var len78 = encodeRes.len;
        
        dataView(memory0).setUint32(base + 32, len78, true);
        dataView(memory0).setUint32(base + 28, ptr78, true);
        var val79 = v73_2;
        var len79 = Array.isArray(val79) ? val79.length : val79.byteLength;
        var ptr79 = await realloc0Async(0, 0, 1, len79 * 1);
        
        let valData79;
        const valLenBytes79 = len79 * 1;
        if (Array.isArray(val79)) {
          // Regular array likely containing numbers, write values to memory
          let offset = 0;
          const dv79 = new DataView(memory0.buffer);
          for (const v of val79) {
            _requireValidNumericPrimitive.bind(null, 'u8')(v);
            dv79.setUint8(ptr79+ offset, v, true);
            offset += 1;
          }
        } else {
          // TypedArray / ArrayBuffer-like, direct copy
          valData79 = new Uint8Array(val79.buffer || val79, val79.byteOffset, valLenBytes79);
          const out79 = new Uint8Array(memory0.buffer, ptr79, valLenBytes79);
          out79.set(valData79);
        }
        
        dataView(memory0).setUint32(base + 40, len79, true);
        dataView(memory0).setUint32(base + 36, ptr79, true);
        break;
      }
      default: {
        throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant80.tag)}\` (received \`${variant80}\`) specified for \`Event\``);
      }
    }
  }
  var {fuel: v82_0, deadlineMs: v82_1, maxEffects: v82_2, maxPatchBytes: v82_3, maxFrames: v82_4 } = arg1;
  _debugLog('[iface="semio:framework/reactor@1.0.0", function="poll"][Instruction::CallWasm] enter', {
    funcName: 'poll',
    paramCount: 7,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'reactor100Poll',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'throw-result-err',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory(() => memory0());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => reactor100Poll(result81, len81, toUint64(v82_0), toUint32(v82_1), toUint32(v82_2), toUint32(v82_3), toUint32(v82_4)),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:framework/reactor@1.0.0", function="poll"][Instruction::AsyncTaskReturn]', {
    funcName: 'poll',
    paramCount: 1,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'poll',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'poll',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
let jobs100StartJob;

async function startJob(arg0, arg1, arg2) {
  
  var encodeRes = await _utf8AllocateAndEncodeAsync(arg1, realloc0Async, memory0);
  var ptr0= encodeRes.ptr;
  var len0 = encodeRes.len;
  
  var val1 = arg2;
  var len1 = Array.isArray(val1) ? val1.length : val1.byteLength;
  var ptr1 = await realloc0Async(0, 0, 1, len1 * 1);
  
  let valData1;
  const valLenBytes1 = len1 * 1;
  if (Array.isArray(val1)) {
    // Regular array likely containing numbers, write values to memory
    let offset = 0;
    const dv1 = new DataView(memory0.buffer);
    for (const v of val1) {
      _requireValidNumericPrimitive.bind(null, 'u8')(v);
      dv1.setUint8(ptr1+ offset, v, true);
      offset += 1;
    }
  } else {
    // TypedArray / ArrayBuffer-like, direct copy
    valData1 = new Uint8Array(val1.buffer || val1, val1.byteOffset, valLenBytes1);
    const out1 = new Uint8Array(memory0.buffer, ptr1, valLenBytes1);
    out1.set(valData1);
  }
  
  _debugLog('[iface="semio:framework/jobs@1.0.0", function="start-job"][Instruction::CallWasm] enter', {
    funcName: 'start-job',
    paramCount: 5,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'jobs100StartJob',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'throw-result-err',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory(() => memory0());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => jobs100StartJob(toUint64(arg0), ptr0, len0, ptr1, len1),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:framework/jobs@1.0.0", function="start-job"][Instruction::AsyncTaskReturn]', {
    funcName: 'start-job',
    paramCount: 1,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'start-job',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'start-job',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
let jobs100StepJob;

async function stepJob(arg0, arg1) {
  var {fuel: v0_0, deadlineMs: v0_1 } = arg1;
  _debugLog('[iface="semio:framework/jobs@1.0.0", function="step-job"][Instruction::CallWasm] enter', {
    funcName: 'step-job',
    paramCount: 3,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'jobs100StepJob',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'throw-result-err',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory(() => memory0());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => jobs100StepJob(toUint64(arg0), toUint64(v0_0), toUint32(v0_1)),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:framework/jobs@1.0.0", function="step-job"][Instruction::AsyncTaskReturn]', {
    funcName: 'step-job',
    paramCount: 1,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'step-job',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'step-job',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
let jobs100CancelJob;

async function cancelJob(arg0) {
  _debugLog('[iface="semio:framework/jobs@1.0.0", function="cancel-job"][Instruction::CallWasm] enter', {
    funcName: 'cancel-job',
    paramCount: 1,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'jobs100CancelJob',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'none',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (null!== null) {
    task.setReturnMemoryIdx(null);
    task.setReturnMemory(() => null());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => jobs100CancelJob(toUint64(arg0)),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:framework/jobs@1.0.0", function="cancel-job"][Instruction::AsyncTaskReturn]', {
    funcName: 'cancel-job',
    paramCount: 0,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'cancel-job',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'cancel-job',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
let checkpoint100Checkpoint;

async function checkpoint() {
  _debugLog('[iface="semio:framework/checkpoint@1.0.0", function="checkpoint"][Instruction::CallWasm] enter', {
    funcName: 'checkpoint',
    paramCount: 0,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'checkpoint100Checkpoint',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'throw-result-err',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory(() => memory0());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => checkpoint100Checkpoint(),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:framework/checkpoint@1.0.0", function="checkpoint"][Instruction::AsyncTaskReturn]', {
    funcName: 'checkpoint',
    paramCount: 1,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'checkpoint',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'checkpoint',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
let checkpoint100Restore;

async function restore(arg0) {
  var val0 = arg0;
  var len0 = Array.isArray(val0) ? val0.length : val0.byteLength;
  var ptr0 = await realloc0Async(0, 0, 1, len0 * 1);
  
  let valData0;
  const valLenBytes0 = len0 * 1;
  if (Array.isArray(val0)) {
    // Regular array likely containing numbers, write values to memory
    let offset = 0;
    const dv0 = new DataView(memory0.buffer);
    for (const v of val0) {
      _requireValidNumericPrimitive.bind(null, 'u8')(v);
      dv0.setUint8(ptr0+ offset, v, true);
      offset += 1;
    }
  } else {
    // TypedArray / ArrayBuffer-like, direct copy
    valData0 = new Uint8Array(val0.buffer || val0, val0.byteOffset, valLenBytes0);
    const out0 = new Uint8Array(memory0.buffer, ptr0, valLenBytes0);
    out0.set(valData0);
  }
  
  _debugLog('[iface="semio:framework/checkpoint@1.0.0", function="restore"][Instruction::CallWasm] enter', {
    funcName: 'restore',
    paramCount: 2,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'checkpoint100Restore',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'throw-result-err',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory(() => memory0());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => checkpoint100Restore(ptr0, len0),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:framework/checkpoint@1.0.0", function="restore"][Instruction::AsyncTaskReturn]', {
    funcName: 'restore',
    paramCount: 1,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'restore',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'restore',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
let describe100Describe;

async function describe() {
  _debugLog('[iface="semio:framework/describe@1.0.0", function="describe"][Instruction::CallWasm] enter', {
    funcName: 'describe',
    paramCount: 0,
    async: true,
    postReturn: false,
  });
  const hostProvided = false;
  
  const [task, _wasm_call_currentTaskID] = createNewCurrentTask({
    componentIdx: 0,
    isAsync: true,
    isManualAsync: false,
    preserveFutureResult: false,
    entryFnName: 'describe100Describe',
    getCallbackFn: () => callback_0,
    callbackFnName: callback_0,
    errHandling: 'none',
    callingWasmExport: true,
  });
  
  
  const started = await task.enter();
  if (!started) {
    _debugLog('[Instruction::AsyncTaskReturn] failed to enter task', {
      taskID: task.id(),
      subtaskID: task.currentSubtask()?.id(),
    });
    throw new Error("failed to enter task");
  }
  
  
  if (0!== null) {
    task.setReturnMemoryIdx(0);
    task.setReturnMemory(() => memory0());
  }
  
  
  let ret;
  
  try {
    ret =  await  _withGlobalCurrentTaskMetaAsync({
      taskID: task.id(),
      componentIdx: task.componentIdx(),
      fn: () => describe100Describe(),
    });
  } catch (err) {
    
    _debugLog('[Instruction::CallWasm] error during async call', {
      taskID: task.id(),
      err,
    });
    task.setErrored(err);
    task.reject(err);
    task.exit();
    return task.completionPromise();
    
  }
  
  _debugLog('[iface="semio:framework/describe@1.0.0", function="describe"][Instruction::AsyncTaskReturn]', {
    funcName: 'describe',
    paramCount: 1,
    componentIdx: 0,
    postReturn: false,
    hostProvided,
  });
  
  if (hostProvided) {
    _debugLog('[Instruction::AsyncTaskReturn] signaling host-provided async return completion', {
      task: task.id(),
      subtask: subtask?.id(),
      result: ret,
    })
    task.resolve([ret]);
    task.exit();
    return await task.completionPromise();
  }
  
  const componentState = getOrCreateAsyncState(0);
  if (!componentState) { throw new Error('failed to lookup current component state'); }
  
  queueMicrotask(async (resolve, reject) => {
    try {
      _debugLog("[Instruction::AsyncTaskReturn] starting driver loop", {
        fnName: 'describe',
        componentInstanceIdx: 0,
        taskID: task.id(),
      });
      await _driverLoop({
        componentInstanceIdx: 0,
        componentState,
        task,
        fnName: 'describe',
        isAsync: true,
        callbackResult: ret,
      });
    } catch (err) {
      _debugLog("[Instruction::AsyncTaskReturn] driver loop call failure", { err });
    }
  });
  
  let taskRes = await task.completionPromise();
  if (task.getErrHandling() === 'throw-result-err') {
    if (typeof taskRes !== 'object') {
      return taskRes;
    }
    if (taskRes.tag === 'err') { throw taskRes.val; }
    if (taskRes.tag === 'ok') { taskRes = taskRes.val; }
  }
  
  return taskRes;
  
}
const trampoline0 = taskReturn.bind(
null,
{
  componentIdx: 0,
  useDirectParams: true,
  getMemoryFn: () => null,
  memoryIdx: null,
  callbackFnIdx: null,
  liftFns: [],
  lowerFns: [],
  stringEncoding: 'utf8',
},
);
let trampoline1 = _trampoline1.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 1,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline1.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatS64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline1,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 1,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline1.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatS64],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline1,
},
);
const trampoline2 = waitableJoin.bind(null, 0);

const trampoline3 = waitableSetNew.bind(null, 0);

const trampoline4 = waitableSetDrop.bind(null, 0);

const trampoline5 = taskCancel.bind(null, 0);

function trampoline6(handle) {
  const handleEntry = rscTableRemove(handleTable1, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable1.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable1.delete(handleEntry.rep);
    } else if (Error$1[symbolCabiDispose]) {
      Error$1[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline7(handle) {
  const handleEntry = rscTableRemove(handleTable0, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable0.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable0.delete(handleEntry.rep);
    } else if (Pollable[symbolCabiDispose]) {
      Pollable[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline8(handle) {
  const handleEntry = rscTableRemove(handleTable2, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable2.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable2.delete(handleEntry.rep);
    } else if (InputStream[symbolCabiDispose]) {
      InputStream[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline9(handle) {
  const handleEntry = rscTableRemove(handleTable3, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable3.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable3.delete(handleEntry.rep);
    } else if (OutputStream[symbolCabiDispose]) {
      OutputStream[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline10(handle) {
  const handleEntry = rscTableRemove(handleTable4, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable4.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable4.delete(handleEntry.rep);
    } else if (TerminalInput[symbolCabiDispose]) {
      TerminalInput[symbolCabiDispose](handleEntry.rep);
    }
  }
}
function trampoline11(handle) {
  const handleEntry = rscTableRemove(handleTable5, handle);
  if (handleEntry.own) {
    
    const rsc = captureTable5.get(handleEntry.rep);
    if (rsc) {
      if (rsc[symbolDispose]) rsc[symbolDispose]();
      captureTable5.delete(handleEntry.rep);
    } else if (TerminalOutput[symbolCabiDispose]) {
      TerminalOutput[symbolCabiDispose](handleEntry.rep);
    }
  }
}
let trampoline12 = _trampoline12.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 12,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline12.manuallyAsync,
  paramLiftFns: [
  _liftFlatResult({
    caseMetas: [['ok', null, 0, 0, 0],['err', null, 0, 0, 0],],
    variantSize32: 1,
    variantAlign32: 1,
    variantPayloadOffset32: 1,
    variantFlatCount: 1,
  })
  ],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline12,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 12,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline12.manuallyAsync,
  paramLiftFns: [
  _liftFlatResult({
    caseMetas: [['ok', null, 0, 0, 0],['err', null, 0, 0, 0],],
    variantSize32: 1,
    variantAlign32: 1,
    variantPayloadOffset32: 1,
    variantFlatCount: 1,
  })
  ],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline12,
},
);
let trampoline13 = _trampoline13.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 13,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline13.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 0)],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline13,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 13,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline13.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 0)],
  resultLowerFns: [],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline13,
},
);
let trampoline14 = _trampoline14.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 14,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline14.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_Pollable(obj) {
      if (!(obj instanceof Pollable)) {
        throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt0;
        captureTable0.set(rep, obj);
        handle = rscTableCreateOwn(handleTable0, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline14,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 14,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline14.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_Pollable(obj) {
      if (!(obj instanceof Pollable)) {
        throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt0;
        captureTable0.set(rep, obj);
        handle = rscTableCreateOwn(handleTable0, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline14,
},
);
let trampoline15 = _trampoline15.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 15,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline15.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_InputStream(obj) {
      if (!(obj instanceof InputStream)) {
        throw new TypeError('Resource error: Not a valid \"InputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt2;
        captureTable2.set(rep, obj);
        handle = rscTableCreateOwn(handleTable2, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline15,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 15,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline15.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_InputStream(obj) {
      if (!(obj instanceof InputStream)) {
        throw new TypeError('Resource error: Not a valid \"InputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt2;
        captureTable2.set(rep, obj);
        handle = rscTableCreateOwn(handleTable2, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline15,
},
);
let trampoline16 = _trampoline16.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 16,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline16.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_OutputStream(obj) {
      if (!(obj instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt3;
        captureTable3.set(rep, obj);
        handle = rscTableCreateOwn(handleTable3, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline16,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 16,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline16.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_OutputStream(obj) {
      if (!(obj instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt3;
        captureTable3.set(rep, obj);
        handle = rscTableCreateOwn(handleTable3, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline16,
},
);
let trampoline17 = _trampoline17.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 17,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline17.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_OutputStream(obj) {
      if (!(obj instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt3;
        captureTable3.set(rep, obj);
        handle = rscTableCreateOwn(handleTable3, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline17,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 17,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline17.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_OutputStream(obj) {
      if (!(obj instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid \"OutputStream\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt3;
        captureTable3.set(rep, obj);
        handle = rscTableCreateOwn(handleTable3, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline17,
},
);
let trampoline18 = _trampoline18.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 18,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline18.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_Pollable(obj) {
      if (!(obj instanceof Pollable)) {
        throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt0;
        captureTable0.set(rep, obj);
        handle = rscTableCreateOwn(handleTable0, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline18,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 18,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline18.manuallyAsync,
  paramLiftFns: [_liftFlatU64],
  resultLowerFns: [_lowerFlatOwn({
    componentIdx: 0,
    lowerFn: 
    function lowerImportedOwnedHost_Pollable(obj) {
      if (!(obj instanceof Pollable)) {
        throw new TypeError('Resource error: Not a valid \"Pollable\" resource.');
      }
      let handle = obj[symbolRscHandle];
      if (!handle) {
        const rep = obj[symbolRscRep] || ++captureCnt0;
        captureTable0.set(rep, obj);
        handle = rscTableCreateOwn(handleTable0, rep);
      }
      return handle;
    }
    ,
  })],
  hasResultPointer: false,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: null,
  stringEncoding: 'utf8',
  getMemoryFn: () => null,
  getReallocFn: undefined,
  importFn: _trampoline18,
},
);
const trampoline19 = taskReturn.bind(
null,
{
  componentIdx: 0,
  useDirectParams: true,
  getMemoryFn: () => memory0,
  memoryIdx: 0,
  callbackFnIdx: null,
  liftFns: [
  _liftFlatResult({
    caseMetas: [['ok', null, 0, 0, 0],['err', _liftFlatVariant({
      caseMetas: [['fault', _liftFlatList({
        elemLiftFn: _liftFlatU8,
        elemAlign32: 1,
        elemSize32: 1,
        typedArray: Uint8Array,
      }), 8, 4, 2],],
      variantSize32: 12,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 3,
    } ), 12, 4, 3],],
    variantSize32: 16,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 4,
  })
  ],
  lowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 16, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'fault', _lowerFlatList({
        elemLowerFn: _lowerFlatU8,
        elemSize32: 1,
        elemAlign32: 1,
      }), 8, 4, 2 ],],
      variantSize32: 12,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 3,
    } ), 16, 4, 4 ],
    ],
    variantSize32: 16,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 4,
  })
  ],
  stringEncoding: 'utf8',
},
);
const trampoline20 = taskReturn.bind(
null,
{
  componentIdx: 0,
  useDirectParams: true,
  getMemoryFn: () => memory0,
  memoryIdx: 0,
  callbackFnIdx: null,
  liftFns: [
  _liftFlatResult({
    caseMetas: [['ok', _liftFlatList({
      elemLiftFn: _liftFlatU8,
      elemAlign32: 1,
      elemSize32: 1,
      typedArray: Uint8Array,
    }), 8, 4, 2],['err', _liftFlatVariant({
      caseMetas: [['fault', _liftFlatList({
        elemLiftFn: _liftFlatU8,
        elemAlign32: 1,
        elemSize32: 1,
        typedArray: Uint8Array,
      }), 8, 4, 2],],
      variantSize32: 12,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 3,
    } ), 12, 4, 3],],
    variantSize32: 16,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 4,
  })
  ],
  lowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', _lowerFlatList({
      elemLowerFn: _lowerFlatU8,
      elemSize32: 1,
      elemAlign32: 1,
    }), 16, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'fault', _lowerFlatList({
        elemLowerFn: _lowerFlatU8,
        elemSize32: 1,
        elemAlign32: 1,
      }), 8, 4, 2 ],],
      variantSize32: 12,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 3,
    } ), 16, 4, 4 ],
    ],
    variantSize32: 16,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 4,
  })
  ],
  stringEncoding: 'utf8',
},
);
const trampoline21 = taskReturn.bind(
null,
{
  componentIdx: 0,
  useDirectParams: true,
  getMemoryFn: () => memory0,
  memoryIdx: 0,
  callbackFnIdx: null,
  liftFns: [
  _liftFlatResult({
    caseMetas: [['ok', _liftFlatVariant({
      caseMetas: [['running', 
      _liftFlatOption({
        caseMetas: [
        ['none', null, 0, 0, 0 ],
        ['some', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4, 2 ],
        ],
        variantSize32: 12,
        variantAlign32: 4,
        variantPayloadOffset32: 4,
        variantFlatCount: 3,
      })
      , 12, 4, 3],['done', _liftFlatList({
        elemLiftFn: _liftFlatU8,
        elemAlign32: 1,
        elemSize32: 1,
        typedArray: Uint8Array,
      }), 8, 4, 2],['failed', _liftFlatList({
        elemLiftFn: _liftFlatU8,
        elemAlign32: 1,
        elemSize32: 1,
        typedArray: Uint8Array,
      }), 8, 4, 2],],
      variantSize32: 16,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 4,
    } ), 16, 4, 4],['err', _liftFlatVariant({
      caseMetas: [['fault', _liftFlatList({
        elemLiftFn: _liftFlatU8,
        elemAlign32: 1,
        elemSize32: 1,
        typedArray: Uint8Array,
      }), 8, 4, 2],],
      variantSize32: 12,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 3,
    } ), 12, 4, 3],],
    variantSize32: 20,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 5,
  })
  ],
  lowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', _lowerFlatVariant({
      caseMetas: [[ 'running', 
      _lowerFlatOption({
        caseMetas: [
        [ 'none', null, 0, 0, 0 ],
        [ 'some', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4, 2],
        ],
        variantSize32: 12,
        variantAlign32: 4,
        variantPayloadOffset32: 4,
        variantFlatCount: 3,
      })
      , 12, 4, 3 ],[ 'done', _lowerFlatList({
        elemLowerFn: _lowerFlatU8,
        elemSize32: 1,
        elemAlign32: 1,
      }), 8, 4, 2 ],[ 'failed', _lowerFlatList({
        elemLowerFn: _lowerFlatU8,
        elemSize32: 1,
        elemAlign32: 1,
      }), 8, 4, 2 ],],
      variantSize32: 16,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 4,
    } ), 20, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'fault', _lowerFlatList({
        elemLowerFn: _lowerFlatU8,
        elemSize32: 1,
        elemAlign32: 1,
      }), 8, 4, 2 ],],
      variantSize32: 12,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 3,
    } ), 20, 4, 4 ],
    ],
    variantSize32: 20,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 5,
  })
  ],
  stringEncoding: 'utf8',
},
);
const trampoline22 = taskReturn.bind(
null,
{
  componentIdx: 0,
  useDirectParams: true,
  getMemoryFn: () => memory0,
  memoryIdx: 0,
  callbackFnIdx: null,
  liftFns: [
  _liftFlatResult({
    caseMetas: [['ok', _liftFlatRecord({ fieldMetas: [['uiPatches', _liftFlatList({
      elemLiftFn: _liftFlatRecord({ fieldMetas: [['surface', _liftFlatRecord({ fieldMetas: [['instance', _liftFlatU32, 4, 4],['surface', _liftFlatU32, 4, 4],], size32: 8, align32: 4 }), 8, 4],['kind', _liftFlatStringAny, 8, 4],['revision', _liftFlatU64, 8, 8],['baseRevision', _liftFlatU64, 8, 8],['ops', _liftFlatList({
        elemLiftFn: _liftFlatVariant({
          caseMetas: [['replace', _liftFlatRecord({ fieldMetas: [['path', _liftFlatList({
            elemLiftFn: _liftFlatU32,
            elemAlign32: 4,
            elemSize32: 4,
            typedArray: Uint32Array,
          }), 8, 4],['node', _liftFlatList({
            elemLiftFn: _liftFlatU8,
            elemAlign32: 1,
            elemSize32: 1,
            typedArray: Uint8Array,
          }), 8, 4],], size32: 16, align32: 4 }), 16, 4, 4],['insert-child', _liftFlatRecord({ fieldMetas: [['path', _liftFlatList({
            elemLiftFn: _liftFlatU32,
            elemAlign32: 4,
            elemSize32: 4,
            typedArray: Uint32Array,
          }), 8, 4],['index', _liftFlatU32, 4, 4],['node', _liftFlatList({
            elemLiftFn: _liftFlatU8,
            elemAlign32: 1,
            elemSize32: 1,
            typedArray: Uint8Array,
          }), 8, 4],], size32: 20, align32: 4 }), 20, 4, 5],['remove-child', _liftFlatRecord({ fieldMetas: [['path', _liftFlatList({
            elemLiftFn: _liftFlatU32,
            elemAlign32: 4,
            elemSize32: 4,
            typedArray: Uint32Array,
          }), 8, 4],['index', _liftFlatU32, 4, 4],], size32: 12, align32: 4 }), 12, 4, 3],['set-props', _liftFlatRecord({ fieldMetas: [['path', _liftFlatList({
            elemLiftFn: _liftFlatU32,
            elemAlign32: 4,
            elemSize32: 4,
            typedArray: Uint32Array,
          }), 8, 4],['props', _liftFlatList({
            elemLiftFn: _liftFlatU8,
            elemAlign32: 1,
            elemSize32: 1,
            typedArray: Uint8Array,
          }), 8, 4],], size32: 16, align32: 4 }), 16, 4, 4],],
          variantSize32: 24,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 6,
        } ),
        elemAlign32: 4,
        elemSize32: 24,
        typedArray: undefined,
      }), 8, 4],], size32: 40, align32: 8 }),
      elemAlign32: 8,
      elemSize32: 40,
      typedArray: undefined,
    }), 8, 4],['effects', _liftFlatList({
      elemLiftFn: _liftFlatVariant({
        caseMetas: [['send-message', _liftFlatRecord({ fieldMetas: [['target', _liftFlatVariant({
          caseMetas: [['shell', _liftFlatU32, 4, 4, 1],['backbone', _liftFlatStringAny, 8, 4, 2],['plugin-instance', _liftFlatU32, 4, 4, 1],['extension', _liftFlatStringAny, 8, 4, 2],['topic', _liftFlatStringAny, 8, 4, 2],],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        } ), 12, 4],['payload', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 20, align32: 4 }), 20, 4, 5],['publish-event', _liftFlatRecord({ fieldMetas: [['topic', _liftFlatStringAny, 8, 4],['payload', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 16, align32: 4 }), 16, 4, 4],['blob-load', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['hash', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4],], size32: 16, align32: 8 }), 16, 8, 3],['blob-write', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['mediaType', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],['bytes', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 16, align32: 4 }), 16, 4],], size32: 24, align32: 8 }), 24, 8, 5],['http-request', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['method', _liftFlatStringAny, 8, 4],['url', _liftFlatStringAny, 8, 4],['headers', _liftFlatList({
          elemLiftFn: _liftFlatTuple({ elemLiftFns: [[_liftFlatStringAny, 8, 4],[_liftFlatStringAny, 8, 4],], size32: 16, align32: 4 }),
          elemAlign32: 4,
          elemSize32: 16,
          typedArray: undefined,
        }), 8, 4],['body', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatList({
            elemLiftFn: _liftFlatU8,
            elemAlign32: 1,
            elemSize32: 1,
            typedArray: Uint8Array,
          }), 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],['streaming', _liftFlatBool, 1, 1],], size32: 40, align32: 4 }), 40, 4],], size32: 48, align32: 8 }), 48, 8, 11],['document-read', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['doc', _liftFlatU64, 8, 8],['lane', _liftFlatStringAny, 8, 4],], size32: 16, align32: 8 }), 16, 8],], size32: 24, align32: 8 }), 24, 8, 4],['document-write', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['doc', _liftFlatU64, 8, 8],['lane', _liftFlatStringAny, 8, 4],['ops', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 24, align32: 8 }), 24, 8],], size32: 32, align32: 8 }), 32, 8, 6],['link-resolve', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['link', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 16, align32: 8 }), 16, 8, 3],['registry-query', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['kind', _liftFlatStringAny, 8, 4],['filter', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 16, align32: 4 }), 16, 4],], size32: 24, align32: 8 }), 24, 8, 5],['io-compose', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['key', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],['sources', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 16, align32: 4 }), 16, 4],], size32: 24, align32: 8 }), 24, 8, 5],['io-run', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['source', _liftFlatStringAny, 8, 4],['target', _liftFlatStringAny, 8, 4],['payload', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 24, align32: 4 }), 24, 4],], size32: 32, align32: 8 }), 32, 8, 7],['cache-derive', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['engineId', _liftFlatStringAny, 8, 4],['input', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 16, align32: 4 }), 16, 4],], size32: 24, align32: 8 }), 24, 8, 5],['cache-read', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['engineId', _liftFlatStringAny, 8, 4],['key', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 16, align32: 4 }), 16, 4],], size32: 24, align32: 8 }), 24, 8, 5],['open-window', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['kind', _liftFlatStringAny, 8, 4],['params', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 16, align32: 4 }), 16, 4],], size32: 24, align32: 8 }), 24, 8, 5],['close-window', _liftFlatRecord({ fieldMetas: [['window', _liftFlatU64, 8, 8],], size32: 8, align32: 8 }), 8, 8, 1],['dispatch-action', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['action', _liftFlatStringAny, 8, 4],['args', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatList({
            elemLiftFn: _liftFlatU8,
            elemAlign32: 1,
            elemSize32: 1,
            typedArray: Uint8Array,
          }), 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],['delayMs', _liftFlatU64, 8, 8],], size32: 32, align32: 8 }), 32, 8],], size32: 40, align32: 8 }), 40, 8, 7],['invoke-extension', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['extensionId', _liftFlatStringAny, 8, 4],['capability', _liftFlatStringAny, 8, 4],['payload', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 24, align32: 4 }), 24, 4],], size32: 32, align32: 8 }), 32, 8, 7],['notify', _liftFlatRecord({ fieldMetas: [['message', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4, 2],['clipboard-write', _liftFlatRecord({ fieldMetas: [['fragment', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 8, align32: 4 }), 8, 4, 2],['navigate', _liftFlatRecord({ fieldMetas: [['uri', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4, 2],['open-external-url', _liftFlatRecord({ fieldMetas: [['url', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4, 2],['set-panel', _liftFlatRecord({ fieldMetas: [['panelJson', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4, 2],['set-active-utility', _liftFlatRecord({ fieldMetas: [['windowId', _liftFlatStringAny, 8, 4],['utilityId', _liftFlatStringAny, 8, 4],], size32: 16, align32: 4 }), 16, 4, 4],['set-active-tool', _liftFlatRecord({ fieldMetas: [['toolId', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4, 2],['patch-world3d-chrome', _liftFlatRecord({ fieldMetas: [['selectionJson', _liftFlatStringAny, 8, 4],['vorticesJson', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatStringAny, 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],['documentSelectedIds', _liftFlatList({
          elemLiftFn: _liftFlatStringAny,
          elemAlign32: 4,
          elemSize32: 8,
          typedArray: undefined,
        }), 8, 4],['documentHighlightedIds', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatList({
            elemLiftFn: _liftFlatStringAny,
            elemAlign32: 4,
            elemSize32: 8,
            typedArray: undefined,
          }), 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],], size32: 40, align32: 4 }), 40, 4, 10],['replay-shell-command', _liftFlatRecord({ fieldMetas: [['actionId', _liftFlatStringAny, 8, 4],['args', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatList({
            elemLiftFn: _liftFlatU8,
            elemAlign32: 1,
            elemSize32: 1,
            typedArray: Uint8Array,
          }), 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],], size32: 20, align32: 4 }), 20, 4, 5],['spawn-plugin-instance', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['pluginId', _liftFlatStringAny, 8, 4],['appId', _liftFlatStringAny, 8, 4],['osInstanceId', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatStringAny, 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],['label', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatStringAny, 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],['documentJson', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatStringAny, 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],], size32: 52, align32: 4 }), 52, 4],], size32: 64, align32: 8 }), 64, 8, 14],['open-plugin-instance', _liftFlatRecord({ fieldMetas: [['pluginId', _liftFlatStringAny, 8, 4],['appId', _liftFlatStringAny, 8, 4],['osInstanceId', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatStringAny, 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],], size32: 28, align32: 4 }), 28, 4, 7],['open-dialog', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['dialogId', _liftFlatStringAny, 8, 4],['args', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatList({
            elemLiftFn: _liftFlatU8,
            elemAlign32: 1,
            elemSize32: 1,
            typedArray: Uint8Array,
          }), 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],], size32: 20, align32: 4 }), 20, 4],], size32: 32, align32: 8 }), 32, 8, 6],['icon-render-export', _liftFlatRecord({ fieldMetas: [['items', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 8, align32: 4 }), 8, 4, 2],['download-media-export', _liftFlatRecord({ fieldMetas: [['filename', _liftFlatStringAny, 8, 4],['mimeType', _liftFlatStringAny, 8, 4],['data', _liftFlatStringAny, 8, 4],['encoding', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatStringAny, 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],], size32: 36, align32: 4 }), 36, 4, 9],['request-file-open', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['accept', _liftFlatStringAny, 8, 4],['readAs', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatStringAny, 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],['importAction', _liftFlatStringAny, 8, 4],['multiple', _liftFlatBool, 1, 1],], size32: 32, align32: 4 }), 32, 4],], size32: 40, align32: 8 }), 40, 8, 9],['request-media-frames', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['accept', _liftFlatStringAny, 8, 4],['frameAction', _liftFlatStringAny, 8, 4],['doneAction', _liftFlatStringAny, 8, 4],['fallbackAction', _liftFlatStringAny, 8, 4],['sampleStride', _liftFlatU32, 4, 4],['maxFrames', _liftFlatU32, 4, 4],['maxLongEdgePx', _liftFlatU32, 4, 4],['fpsHint', _liftFlatFloat64, 8, 8],['payload', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatStringAny, 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],['args', 
        _liftFlatOption({
          caseMetas: [
          ['none', null, 0, 0, 0 ],
          ['some', _liftFlatList({
            elemLiftFn: _liftFlatU8,
            elemAlign32: 1,
            elemSize32: 1,
            typedArray: Uint8Array,
          }), 8, 4, 2 ],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4],], size32: 80, align32: 8 }), 80, 8],], size32: 88, align32: 8 }), 88, 8, null],['load-document', _liftFlatRecord({ fieldMetas: [['docPack', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],['spr', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 16, align32: 4 }), 16, 4, 4],['request-sync', null, 0, 0, 0],['set-timer', _liftFlatRecord({ fieldMetas: [['id', _liftFlatU64, 8, 8],['afterMs', _liftFlatU32, 4, 4],['repeat', _liftFlatBool, 1, 1],], size32: 16, align32: 8 }), 16, 8, 3],['spawn-job', _liftFlatRecord({ fieldMetas: [['job', _liftFlatU64, 8, 8],['kind', _liftFlatStringAny, 8, 4],['input', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],['placement', 
        _liftFlatEnum({
          caseMetas: [['inline', null, 1, 1, 1],['isolated', null, 1, 1, 1],['exclusive', null, 1, 1, 1],],
          variantSize32: 1,
          variantAlign32: 1,
          variantPayloadOffset32: 1,
          variantFlatCount: 1,
        })
        , 1, 1],], size32: 32, align32: 8 }), 32, 8, 6],['cancel-job', _liftFlatRecord({ fieldMetas: [['job', _liftFlatU64, 8, 8],], size32: 8, align32: 8 }), 8, 8, 1],['respond', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['outcome', _liftFlatVariant({
          caseMetas: [['ok', _liftFlatList({
            elemLiftFn: _liftFlatU8,
            elemAlign32: 1,
            elemSize32: 1,
            typedArray: Uint8Array,
          }), 8, 4, 2],['fault', _liftFlatList({
            elemLiftFn: _liftFlatU8,
            elemAlign32: 1,
            elemSize32: 1,
            typedArray: Uint8Array,
          }), 8, 4, 2],],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        } ), 12, 4],], size32: 24, align32: 8 }), 24, 8, 4],['storage-read', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['key', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4],], size32: 16, align32: 8 }), 16, 8, 3],['storage-write', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['key', _liftFlatStringAny, 8, 4],['value', _liftFlatList({
          elemLiftFn: _liftFlatU8,
          elemAlign32: 1,
          elemSize32: 1,
          typedArray: Uint8Array,
        }), 8, 4],], size32: 16, align32: 4 }), 16, 4],], size32: 24, align32: 8 }), 24, 8, 5],['storage-delete', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['key', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4],], size32: 16, align32: 8 }), 16, 8, 3],['request-capability', _liftFlatRecord({ fieldMetas: [['req', _liftFlatU64, 8, 8],['params', _liftFlatRecord({ fieldMetas: [['id', _liftFlatStringAny, 8, 4],['scope', _liftFlatStringAny, 8, 4],['reason', _liftFlatStringAny, 8, 4],['optional', _liftFlatBool, 1, 1],], size32: 28, align32: 4 }), 28, 4],], size32: 40, align32: 8 }), 40, 8, 8],['release-capability', _liftFlatRecord({ fieldMetas: [['id', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4, 2],['subscribe', _liftFlatRecord({ fieldMetas: [['topic', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4, 2],['unsubscribe', _liftFlatRecord({ fieldMetas: [['topic', _liftFlatStringAny, 8, 4],], size32: 8, align32: 4 }), 8, 4, 2],],
        variantSize32: 96,
        variantAlign32: 8,
        variantPayloadOffset32: 8,
        variantFlatCount: null,
      } ),
      elemAlign32: 8,
      elemSize32: 96,
      typedArray: undefined,
    }), 8, 4],['nextWake', 
    _liftFlatOption({
      caseMetas: [
      ['none', null, 0, 0, 0 ],
      ['some', _liftFlatU64, 8, 8, 1 ],
      ],
      variantSize32: 16,
      variantAlign32: 8,
      variantPayloadOffset32: 8,
      variantFlatCount: 2,
    })
    , 16, 8],['status', _liftFlatVariant({
      caseMetas: [['idle', null, 0, 0, 0],['more-work', null, 0, 0, 0],['checkpoint-ready', null, 0, 0, 0],['faulted', _liftFlatList({
        elemLiftFn: _liftFlatU8,
        elemAlign32: 1,
        elemSize32: 1,
        typedArray: Uint8Array,
      }), 8, 4, 2],],
      variantSize32: 12,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 3,
    } ), 12, 4],['fuelUsed', _liftFlatU64, 8, 8],], size32: 56, align32: 8 }), 56, 8, 10],['err', _liftFlatVariant({
      caseMetas: [['fault', _liftFlatList({
        elemLiftFn: _liftFlatU8,
        elemAlign32: 1,
        elemSize32: 1,
        typedArray: Uint8Array,
      }), 8, 4, 2],],
      variantSize32: 12,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 3,
    } ), 12, 4, 3],],
    variantSize32: 64,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 11,
  })
  ],
  lowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', _lowerFlatRecord({ fieldMetas: [['uiPatches', _lowerFlatList({
      elemLowerFn: _lowerFlatRecord({ fieldMetas: [['surface', _lowerFlatRecord({ fieldMetas: [['instance', _lowerFlatU32, 4, 4 ],['surface', _lowerFlatU32, 4, 4 ],], size32: 8, align32: 4 }), 8, 4 ],['kind', _lowerFlatStringAny, 8, 4 ],['revision', _lowerFlatU64, 8, 8 ],['baseRevision', _lowerFlatU64, 8, 8 ],['ops', _lowerFlatList({
        elemLowerFn: _lowerFlatVariant({
          caseMetas: [[ 'replace', _lowerFlatRecord({ fieldMetas: [['path', _lowerFlatList({
            elemLowerFn: _lowerFlatU32,
            elemSize32: 4,
            elemAlign32: 4,
          }), 8, 4 ],['node', _lowerFlatList({
            elemLowerFn: _lowerFlatU8,
            elemSize32: 1,
            elemAlign32: 1,
          }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4, 4 ],[ 'insert-child', _lowerFlatRecord({ fieldMetas: [['path', _lowerFlatList({
            elemLowerFn: _lowerFlatU32,
            elemSize32: 4,
            elemAlign32: 4,
          }), 8, 4 ],['index', _lowerFlatU32, 4, 4 ],['node', _lowerFlatList({
            elemLowerFn: _lowerFlatU8,
            elemSize32: 1,
            elemAlign32: 1,
          }), 8, 4 ],], size32: 20, align32: 4 }), 20, 4, 5 ],[ 'remove-child', _lowerFlatRecord({ fieldMetas: [['path', _lowerFlatList({
            elemLowerFn: _lowerFlatU32,
            elemSize32: 4,
            elemAlign32: 4,
          }), 8, 4 ],['index', _lowerFlatU32, 4, 4 ],], size32: 12, align32: 4 }), 12, 4, 3 ],[ 'set-props', _lowerFlatRecord({ fieldMetas: [['path', _lowerFlatList({
            elemLowerFn: _lowerFlatU32,
            elemSize32: 4,
            elemAlign32: 4,
          }), 8, 4 ],['props', _lowerFlatList({
            elemLowerFn: _lowerFlatU8,
            elemSize32: 1,
            elemAlign32: 1,
          }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4, 4 ],],
          variantSize32: 24,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 6,
        } ),
        elemSize32: 24,
        elemAlign32: 4,
      }), 8, 4 ],], size32: 40, align32: 8 }),
      elemSize32: 40,
      elemAlign32: 8,
    }), 8, 4 ],['effects', _lowerFlatList({
      elemLowerFn: _lowerFlatVariant({
        caseMetas: [[ 'send-message', _lowerFlatRecord({ fieldMetas: [['target', _lowerFlatVariant({
          caseMetas: [[ 'shell', _lowerFlatU32, 4, 4, 1 ],[ 'backbone', _lowerFlatStringAny, 8, 4, 2 ],[ 'plugin-instance', _lowerFlatU32, 4, 4, 1 ],[ 'extension', _lowerFlatStringAny, 8, 4, 2 ],[ 'topic', _lowerFlatStringAny, 8, 4, 2 ],],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        } ), 12, 4 ],['payload', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 20, align32: 4 }), 20, 4, 5 ],[ 'publish-event', _lowerFlatRecord({ fieldMetas: [['topic', _lowerFlatStringAny, 8, 4 ],['payload', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4, 4 ],[ 'blob-load', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['hash', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4 ],], size32: 16, align32: 8 }), 16, 8, 3 ],[ 'blob-write', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['mediaType', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],['bytes', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4 ],], size32: 24, align32: 8 }), 24, 8, 5 ],[ 'http-request', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['method', _lowerFlatStringAny, 8, 4 ],['url', _lowerFlatStringAny, 8, 4 ],['headers', _lowerFlatList({
          elemLowerFn: _lowerFlatTuple({ elemLowerMetas: [[_lowerFlatStringAny, 8, 4],[_lowerFlatStringAny, 8, 4],], size32: 16, align32: 4 }),
          elemSize32: 16,
          elemAlign32: 4,
        }), 8, 4 ],['body', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatList({
            elemLowerFn: _lowerFlatU8,
            elemSize32: 1,
            elemAlign32: 1,
          }), 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],['streaming', _lowerFlatBool, 1, 1 ],], size32: 40, align32: 4 }), 40, 4 ],], size32: 48, align32: 8 }), 48, 8, 11 ],[ 'document-read', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['doc', _lowerFlatU64, 8, 8 ],['lane', _lowerFlatStringAny, 8, 4 ],], size32: 16, align32: 8 }), 16, 8 ],], size32: 24, align32: 8 }), 24, 8, 4 ],[ 'document-write', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['doc', _lowerFlatU64, 8, 8 ],['lane', _lowerFlatStringAny, 8, 4 ],['ops', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 24, align32: 8 }), 24, 8 ],], size32: 32, align32: 8 }), 32, 8, 6 ],[ 'link-resolve', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['link', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 16, align32: 8 }), 16, 8, 3 ],[ 'registry-query', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['kind', _lowerFlatStringAny, 8, 4 ],['filter', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4 ],], size32: 24, align32: 8 }), 24, 8, 5 ],[ 'io-compose', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['key', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],['sources', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4 ],], size32: 24, align32: 8 }), 24, 8, 5 ],[ 'io-run', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['source', _lowerFlatStringAny, 8, 4 ],['target', _lowerFlatStringAny, 8, 4 ],['payload', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 24, align32: 4 }), 24, 4 ],], size32: 32, align32: 8 }), 32, 8, 7 ],[ 'cache-derive', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['engineId', _lowerFlatStringAny, 8, 4 ],['input', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4 ],], size32: 24, align32: 8 }), 24, 8, 5 ],[ 'cache-read', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['engineId', _lowerFlatStringAny, 8, 4 ],['key', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4 ],], size32: 24, align32: 8 }), 24, 8, 5 ],[ 'open-window', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['kind', _lowerFlatStringAny, 8, 4 ],['params', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4 ],], size32: 24, align32: 8 }), 24, 8, 5 ],[ 'close-window', _lowerFlatRecord({ fieldMetas: [['window', _lowerFlatU64, 8, 8 ],], size32: 8, align32: 8 }), 8, 8, 1 ],[ 'dispatch-action', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['action', _lowerFlatStringAny, 8, 4 ],['args', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatList({
            elemLowerFn: _lowerFlatU8,
            elemSize32: 1,
            elemAlign32: 1,
          }), 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],['delayMs', _lowerFlatU64, 8, 8 ],], size32: 32, align32: 8 }), 32, 8 ],], size32: 40, align32: 8 }), 40, 8, 7 ],[ 'invoke-extension', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['extensionId', _lowerFlatStringAny, 8, 4 ],['capability', _lowerFlatStringAny, 8, 4 ],['payload', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 24, align32: 4 }), 24, 4 ],], size32: 32, align32: 8 }), 32, 8, 7 ],[ 'notify', _lowerFlatRecord({ fieldMetas: [['message', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4, 2 ],[ 'clipboard-write', _lowerFlatRecord({ fieldMetas: [['fragment', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 8, align32: 4 }), 8, 4, 2 ],[ 'navigate', _lowerFlatRecord({ fieldMetas: [['uri', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4, 2 ],[ 'open-external-url', _lowerFlatRecord({ fieldMetas: [['url', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4, 2 ],[ 'set-panel', _lowerFlatRecord({ fieldMetas: [['panelJson', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4, 2 ],[ 'set-active-utility', _lowerFlatRecord({ fieldMetas: [['windowId', _lowerFlatStringAny, 8, 4 ],['utilityId', _lowerFlatStringAny, 8, 4 ],], size32: 16, align32: 4 }), 16, 4, 4 ],[ 'set-active-tool', _lowerFlatRecord({ fieldMetas: [['toolId', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4, 2 ],[ 'patch-world3d-chrome', _lowerFlatRecord({ fieldMetas: [['selectionJson', _lowerFlatStringAny, 8, 4 ],['vorticesJson', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatStringAny, 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],['documentSelectedIds', _lowerFlatList({
          elemLowerFn: _lowerFlatStringAny,
          elemSize32: 8,
          elemAlign32: 4,
        }), 8, 4 ],['documentHighlightedIds', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatList({
            elemLowerFn: _lowerFlatStringAny,
            elemSize32: 8,
            elemAlign32: 4,
          }), 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],], size32: 40, align32: 4 }), 40, 4, 10 ],[ 'replay-shell-command', _lowerFlatRecord({ fieldMetas: [['actionId', _lowerFlatStringAny, 8, 4 ],['args', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatList({
            elemLowerFn: _lowerFlatU8,
            elemSize32: 1,
            elemAlign32: 1,
          }), 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],], size32: 20, align32: 4 }), 20, 4, 5 ],[ 'spawn-plugin-instance', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['pluginId', _lowerFlatStringAny, 8, 4 ],['appId', _lowerFlatStringAny, 8, 4 ],['osInstanceId', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatStringAny, 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],['label', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatStringAny, 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],['documentJson', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatStringAny, 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],], size32: 52, align32: 4 }), 52, 4 ],], size32: 64, align32: 8 }), 64, 8, 14 ],[ 'open-plugin-instance', _lowerFlatRecord({ fieldMetas: [['pluginId', _lowerFlatStringAny, 8, 4 ],['appId', _lowerFlatStringAny, 8, 4 ],['osInstanceId', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatStringAny, 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],], size32: 28, align32: 4 }), 28, 4, 7 ],[ 'open-dialog', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['dialogId', _lowerFlatStringAny, 8, 4 ],['args', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatList({
            elemLowerFn: _lowerFlatU8,
            elemSize32: 1,
            elemAlign32: 1,
          }), 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],], size32: 20, align32: 4 }), 20, 4 ],], size32: 32, align32: 8 }), 32, 8, 6 ],[ 'icon-render-export', _lowerFlatRecord({ fieldMetas: [['items', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 8, align32: 4 }), 8, 4, 2 ],[ 'download-media-export', _lowerFlatRecord({ fieldMetas: [['filename', _lowerFlatStringAny, 8, 4 ],['mimeType', _lowerFlatStringAny, 8, 4 ],['data', _lowerFlatStringAny, 8, 4 ],['encoding', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatStringAny, 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],], size32: 36, align32: 4 }), 36, 4, 9 ],[ 'request-file-open', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['accept', _lowerFlatStringAny, 8, 4 ],['readAs', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatStringAny, 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],['importAction', _lowerFlatStringAny, 8, 4 ],['multiple', _lowerFlatBool, 1, 1 ],], size32: 32, align32: 4 }), 32, 4 ],], size32: 40, align32: 8 }), 40, 8, 9 ],[ 'request-media-frames', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['accept', _lowerFlatStringAny, 8, 4 ],['frameAction', _lowerFlatStringAny, 8, 4 ],['doneAction', _lowerFlatStringAny, 8, 4 ],['fallbackAction', _lowerFlatStringAny, 8, 4 ],['sampleStride', _lowerFlatU32, 4, 4 ],['maxFrames', _lowerFlatU32, 4, 4 ],['maxLongEdgePx', _lowerFlatU32, 4, 4 ],['fpsHint', _lowerFlatFloat64, 8, 8 ],['payload', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatStringAny, 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],['args', 
        _lowerFlatOption({
          caseMetas: [
          [ 'none', null, 0, 0, 0 ],
          [ 'some', _lowerFlatList({
            elemLowerFn: _lowerFlatU8,
            elemSize32: 1,
            elemAlign32: 1,
          }), 8, 4, 2],
          ],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        })
        , 12, 4 ],], size32: 80, align32: 8 }), 80, 8 ],], size32: 88, align32: 8 }), 88, 8, null ],[ 'load-document', _lowerFlatRecord({ fieldMetas: [['docPack', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],['spr', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4, 4 ],[ 'request-sync', null, 0, 0, 0 ],[ 'set-timer', _lowerFlatRecord({ fieldMetas: [['id', _lowerFlatU64, 8, 8 ],['afterMs', _lowerFlatU32, 4, 4 ],['repeat', _lowerFlatBool, 1, 1 ],], size32: 16, align32: 8 }), 16, 8, 3 ],[ 'spawn-job', _lowerFlatRecord({ fieldMetas: [['job', _lowerFlatU64, 8, 8 ],['kind', _lowerFlatStringAny, 8, 4 ],['input', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],['placement', 
        _lowerFlatEnum({
          caseMetas: [['inline', null, 1, 1, 1],['isolated', null, 1, 1, 1],['exclusive', null, 1, 1, 1],],
          variantSize32: 1,
          variantAlign32: 1,
          variantPayloadOffset32: 1,
          variantFlatCount: 1,
        })
        , 1, 1 ],], size32: 32, align32: 8 }), 32, 8, 6 ],[ 'cancel-job', _lowerFlatRecord({ fieldMetas: [['job', _lowerFlatU64, 8, 8 ],], size32: 8, align32: 8 }), 8, 8, 1 ],[ 'respond', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['outcome', _lowerFlatVariant({
          caseMetas: [[ 'ok', _lowerFlatList({
            elemLowerFn: _lowerFlatU8,
            elemSize32: 1,
            elemAlign32: 1,
          }), 8, 4, 2 ],[ 'fault', _lowerFlatList({
            elemLowerFn: _lowerFlatU8,
            elemSize32: 1,
            elemAlign32: 1,
          }), 8, 4, 2 ],],
          variantSize32: 12,
          variantAlign32: 4,
          variantPayloadOffset32: 4,
          variantFlatCount: 3,
        } ), 12, 4 ],], size32: 24, align32: 8 }), 24, 8, 4 ],[ 'storage-read', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['key', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4 ],], size32: 16, align32: 8 }), 16, 8, 3 ],[ 'storage-write', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['key', _lowerFlatStringAny, 8, 4 ],['value', _lowerFlatList({
          elemLowerFn: _lowerFlatU8,
          elemSize32: 1,
          elemAlign32: 1,
        }), 8, 4 ],], size32: 16, align32: 4 }), 16, 4 ],], size32: 24, align32: 8 }), 24, 8, 5 ],[ 'storage-delete', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['key', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4 ],], size32: 16, align32: 8 }), 16, 8, 3 ],[ 'request-capability', _lowerFlatRecord({ fieldMetas: [['req', _lowerFlatU64, 8, 8 ],['params', _lowerFlatRecord({ fieldMetas: [['id', _lowerFlatStringAny, 8, 4 ],['scope', _lowerFlatStringAny, 8, 4 ],['reason', _lowerFlatStringAny, 8, 4 ],['optional', _lowerFlatBool, 1, 1 ],], size32: 28, align32: 4 }), 28, 4 ],], size32: 40, align32: 8 }), 40, 8, 8 ],[ 'release-capability', _lowerFlatRecord({ fieldMetas: [['id', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4, 2 ],[ 'subscribe', _lowerFlatRecord({ fieldMetas: [['topic', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4, 2 ],[ 'unsubscribe', _lowerFlatRecord({ fieldMetas: [['topic', _lowerFlatStringAny, 8, 4 ],], size32: 8, align32: 4 }), 8, 4, 2 ],],
        variantSize32: 96,
        variantAlign32: 8,
        variantPayloadOffset32: 8,
        variantFlatCount: null,
      } ),
      elemSize32: 96,
      elemAlign32: 8,
    }), 8, 4 ],['nextWake', 
    _lowerFlatOption({
      caseMetas: [
      [ 'none', null, 0, 0, 0 ],
      [ 'some', _lowerFlatU64, 8, 8, 1],
      ],
      variantSize32: 16,
      variantAlign32: 8,
      variantPayloadOffset32: 8,
      variantFlatCount: 2,
    })
    , 16, 8 ],['status', _lowerFlatVariant({
      caseMetas: [[ 'idle', null, 0, 0, 0 ],[ 'more-work', null, 0, 0, 0 ],[ 'checkpoint-ready', null, 0, 0, 0 ],[ 'faulted', _lowerFlatList({
        elemLowerFn: _lowerFlatU8,
        elemSize32: 1,
        elemAlign32: 1,
      }), 8, 4, 2 ],],
      variantSize32: 12,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 3,
    } ), 12, 4 ],['fuelUsed', _lowerFlatU64, 8, 8 ],], size32: 56, align32: 8 }), 64, 8, 8 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'fault', _lowerFlatList({
        elemLowerFn: _lowerFlatU8,
        elemSize32: 1,
        elemAlign32: 1,
      }), 8, 4, 2 ],],
      variantSize32: 12,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 3,
    } ), 64, 8, 8 ],
    ],
    variantSize32: 64,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 11,
  })
  ],
  stringEncoding: 'utf8',
},
);
const trampoline23 = taskReturn.bind(
null,
{
  componentIdx: 0,
  useDirectParams: true,
  getMemoryFn: () => memory0,
  memoryIdx: 0,
  callbackFnIdx: null,
  liftFns: [_liftFlatList({
    elemLiftFn: _liftFlatU8,
    elemAlign32: 1,
    elemSize32: 1,
    typedArray: Uint8Array,
  })],
  lowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatU8,
    elemSize32: 1,
    elemAlign32: 1,
  })],
  stringEncoding: 'utf8',
},
);

const trampoline24 = waitableSetPoll.bind(
null,
{
  componentIdx: 0,
  isAsync: false,
  isCancellable: false,
  memoryIdx: 0,
  getMemoryFn: () => memory0,
}
);

let trampoline25 = _trampoline25.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 25,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline25.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatTuple({ elemLowerMetas: [[_lowerFlatU64, 8, 8],[_lowerFlatU64, 8, 8],], size32: 16, align32: 8 })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline25,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 25,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline25.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatTuple({ elemLowerMetas: [[_lowerFlatU64, 8, 8],[_lowerFlatU64, 8, 8],], size32: 16, align32: 8 })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline25,
},
);
let trampoline26 = _trampoline26.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 26,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline26.manuallyAsync,
  paramLiftFns: [_liftFlatList({
    elemLiftFn: _liftFlatBorrow.bind(null, 0),
    elemAlign32: 4,
    elemSize32: 4,
    typedArray: undefined,
  })],
  resultLowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatU32,
    elemSize32: 4,
    elemAlign32: 4,
  })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline26,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 26,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline26.manuallyAsync,
  paramLiftFns: [_liftFlatList({
    elemLiftFn: _liftFlatBorrow.bind(null, 0),
    elemAlign32: 4,
    elemSize32: 4,
    typedArray: undefined,
  })],
  resultLowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatU32,
    elemSize32: 4,
    elemAlign32: 4,
  })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline26,
},
);
let trampoline27 = _trampoline27.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 27,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline27.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', _lowerFlatU64, 16, 8, 8 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 16, 8, 8 ],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline27,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 27,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline27.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', _lowerFlatU64, 16, 8, 8 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 16, 8, 8 ],
    ],
    variantSize32: 16,
    variantAlign32: 8,
    variantPayloadOffset32: 8,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline27,
},
);
let trampoline28 = _trampoline28.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 28,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline28.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3),_liftFlatList({
    elemLiftFn: _liftFlatU8,
    elemAlign32: 1,
    elemSize32: 1,
    typedArray: Uint8Array,
  })],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline28,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 28,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline28.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3),_liftFlatList({
    elemLiftFn: _liftFlatU8,
    elemAlign32: 1,
    elemSize32: 1,
    typedArray: Uint8Array,
  })],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline28,
},
);
let trampoline29 = _trampoline29.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 29,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline29.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline29,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 29,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline29.manuallyAsync,
  paramLiftFns: [_liftFlatBorrow.bind(null, 3)],
  resultLowerFns: [
  _lowerFlatResult({
    caseMetas: [
    [ 'ok', null, 12, 4, 4 ],
    [ 'err', _lowerFlatVariant({
      caseMetas: [[ 'last-operation-failed', _lowerFlatOwn({
        componentIdx: 0,
        lowerFn: 
        function lowerImportedOwnedHost_Error$1(obj) {
          if (!(obj instanceof Error$1)) {
            throw new TypeError('Resource error: Not a valid \"Error$1\" resource.');
          }
          let handle = obj[symbolRscHandle];
          if (!handle) {
            const rep = obj[symbolRscRep] || ++captureCnt1;
            captureTable1.set(rep, obj);
            handle = rscTableCreateOwn(handleTable1, rep);
          }
          return handle;
        }
        ,
      }), 4, 4, 1 ],[ 'closed', null, 0, 0, 0 ],],
      variantSize32: 8,
      variantAlign32: 4,
      variantPayloadOffset32: 4,
      variantFlatCount: 2,
    } ), 12, 4, 4 ],
    ],
    variantSize32: 12,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 3,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline29,
},
);
let trampoline30 = _trampoline30.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 30,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline30.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatTuple({ elemLowerMetas: [[_lowerFlatStringAny, 8, 4],[_lowerFlatStringAny, 8, 4],], size32: 16, align32: 4 }),
    elemSize32: 16,
    elemAlign32: 4,
  })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline30,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 30,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline30.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [_lowerFlatList({
    elemLowerFn: _lowerFlatTuple({ elemLowerMetas: [[_lowerFlatStringAny, 8, 4],[_lowerFlatStringAny, 8, 4],], size32: 16, align32: 4 }),
    elemSize32: 16,
    elemAlign32: 4,
  })],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: () => realloc0,
  importFn: _trampoline30,
},
);
let trampoline31 = _trampoline31.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 31,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline31.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalInput(obj) {
        if (!(obj instanceof TerminalInput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalInput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt4;
          captureTable4.set(rep, obj);
          handle = rscTableCreateOwn(handleTable4, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline31,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 31,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline31.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalInput(obj) {
        if (!(obj instanceof TerminalInput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalInput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt4;
          captureTable4.set(rep, obj);
          handle = rscTableCreateOwn(handleTable4, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline31,
},
);
let trampoline32 = _trampoline32.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 32,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline32.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalOutput(obj) {
        if (!(obj instanceof TerminalOutput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt5;
          captureTable5.set(rep, obj);
          handle = rscTableCreateOwn(handleTable5, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline32,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 32,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline32.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalOutput(obj) {
        if (!(obj instanceof TerminalOutput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt5;
          captureTable5.set(rep, obj);
          handle = rscTableCreateOwn(handleTable5, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline32,
},
);
let trampoline33 = _trampoline33.manuallyAsync ? new WebAssembly.Suspending(_lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 33,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline33.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalOutput(obj) {
        if (!(obj instanceof TerminalOutput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt5;
          captureTable5.set(rep, obj);
          handle = rscTableCreateOwn(handleTable5, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline33,
},
)) : _lowerImportBackwardsCompat.bind(
null,
{
  trampolineIdx: 33,
  componentIdx: 0,
  isAsync: false,
  isManualAsync: _trampoline33.manuallyAsync,
  paramLiftFns: [],
  resultLowerFns: [
  _lowerFlatOption({
    caseMetas: [
    [ 'none', null, 0, 0, 0 ],
    [ 'some', _lowerFlatOwn({
      componentIdx: 0,
      lowerFn: 
      function lowerImportedOwnedHost_TerminalOutput(obj) {
        if (!(obj instanceof TerminalOutput)) {
          throw new TypeError('Resource error: Not a valid \"TerminalOutput\" resource.');
        }
        let handle = obj[symbolRscHandle];
        if (!handle) {
          const rep = obj[symbolRscRep] || ++captureCnt5;
          captureTable5.set(rep, obj);
          handle = rscTableCreateOwn(handleTable5, rep);
        }
        return handle;
      }
      ,
    }), 4, 4, 1],
    ],
    variantSize32: 8,
    variantAlign32: 4,
    variantPayloadOffset32: 4,
    variantFlatCount: 2,
  })
  ],
  hasResultPointer: true,
  funcTypeIsAsync: false,
  getCallbackFn: () => null,
  getPostReturnFn: () => null,
  isCancellable: false,
  memoryIdx: 0,
  stringEncoding: 'utf8',
  getMemoryFn: () => memory0,
  getReallocFn: undefined,
  importFn: _trampoline33,
},
);

const $init = (() => {
  let gen = (function* _initGenerator () {
    const module0 = fetchCompile(new URL('./scalefixture.core.wasm', import.meta.url));
    const module1 = base64Compile('AGFzbQEAAAABRgpgBH9/f38AYAR/f39/AGAFf39/f38AYAt/f39/f39+f39/fgBgAn9/AGACf38Bf2ABfwBgA39/fwBgAn9/AGAEf39/fwADERAAAQIAAwQFBgcICQgGBgYGBAUBcAEQEAdSEQEwAAABMQABATIAAgEzAAMBNAAEATUABQE2AAYBNwAHATgACAE5AAkCMTAACgIxMQALAjEyAAwCMTMADQIxNAAOAjE1AA8IJGltcG9ydHMBAArhARAPACAAIAEgAiADQQARAAALDwAgACABIAIgA0EBEQEACxEAIAAgASACIAMgBEECEQIACw8AIAAgASACIANBAxEAAAsdACAAIAEgAiADIAQgBSAGIAcgCCAJIApBBBEDAAsLACAAIAFBBREEAAsLACAAIAFBBhEFAAsJACAAQQcRBgALDQAgACABIAJBCBEHAAsLACAAIAFBCREIAAsPACAAIAEgAiADQQoRCQALCwAgACABQQsRCAALCQAgAEEMEQYACwkAIABBDREGAAsJACAAQQ4RBgALCQAgAEEPEQYACwAvCXByb2R1Y2VycwEMcHJvY2Vzc2VkLWJ5AQ13aXQtY29tcG9uZW50BzAuMjUyLjA');
    const module2 = base64Compile('AGFzbQEAAAABRgpgBH9/f38AYAR/f39/AGAFf39/f38AYAt/f39/f39+f39/fgBgAn9/AGACf38Bf2ABfwBgA39/fwBgAn9/AGAEf39/fwACZhEAATAAAAABMQABAAEyAAIAATMAAAABNAADAAE1AAQAATYABQABNwAGAAE4AAcAATkACAACMTAACQACMTEACAACMTIABgACMTMABgACMTQABgACMTUABgAIJGltcG9ydHMBcAEQEAkWAQBBAAsQAAECAwQFBgcICQoLDA0ODwAvCXByb2R1Y2VycwEMcHJvY2Vzc2VkLWJ5AQ13aXQtY29tcG9uZW50BzAuMjUyLjA');
    ({ exports: exports0 } = yield instantiateCore(yield module1));
    ({ exports: exports1 } = yield instantiateCore(yield module0, {
      $root: {
        '[context-get-0]': contextGet.bind(null, { componentIdx: 0, slot: 0 }),
        '[context-set-0]': contextSet.bind(null, { componentIdx: 0, slot: 0 }),
        '[waitable-join]': trampoline2,
        '[waitable-set-drop]': trampoline4,
        '[waitable-set-new]': trampoline3,
        '[waitable-set-poll]': exports0['6'],
      },
      '[export]$root': {
        '[task-cancel]': trampoline5,
      },
      '[export]semio:framework/checkpoint@1.0.0': {
        '[task-return]checkpoint': exports0['1'],
        '[task-return]restore': exports0['0'],
      },
      '[export]semio:framework/describe@1.0.0': {
        '[task-return]describe': exports0['5'],
      },
      '[export]semio:framework/jobs@1.0.0': {
        '[task-return]cancel-job': trampoline0,
        '[task-return]start-job': exports0['3'],
        '[task-return]step-job': exports0['2'],
      },
      '[export]semio:framework/reactor@1.0.0': {
        '[task-return]poll': exports0['4'],
      },
      'semio:framework/pure@1.0.0': {
        'now-ms': trampoline1,
      },
      'wasi:cli/environment@0.2.0': {
        'get-environment': exports0['12'],
      },
      'wasi:cli/exit@0.2.0': {
        exit: trampoline12,
      },
      'wasi:cli/stderr@0.2.0': {
        'get-stderr': trampoline17,
      },
      'wasi:cli/stdin@0.2.0': {
        'get-stdin': trampoline15,
      },
      'wasi:cli/stdout@0.2.0': {
        'get-stdout': trampoline16,
      },
      'wasi:cli/terminal-input@0.2.0': {
        '[resource-drop]terminal-input': trampoline10,
      },
      'wasi:cli/terminal-output@0.2.0': {
        '[resource-drop]terminal-output': trampoline11,
      },
      'wasi:cli/terminal-stderr@0.2.0': {
        'get-terminal-stderr': exports0['15'],
      },
      'wasi:cli/terminal-stdin@0.2.0': {
        'get-terminal-stdin': exports0['13'],
      },
      'wasi:cli/terminal-stdout@0.2.0': {
        'get-terminal-stdout': exports0['14'],
      },
      'wasi:clocks/monotonic-clock@0.2.0': {
        'subscribe-duration': trampoline18,
      },
      'wasi:io/error@0.2.0': {
        '[resource-drop]error': trampoline6,
      },
      'wasi:io/poll@0.2.0': {
        '[method]pollable.block': trampoline13,
        '[resource-drop]pollable': trampoline7,
        poll: exports0['8'],
      },
      'wasi:io/streams@0.2.0': {
        '[method]output-stream.blocking-flush': exports0['11'],
        '[method]output-stream.check-write': exports0['9'],
        '[method]output-stream.subscribe': trampoline14,
        '[method]output-stream.write': exports0['10'],
        '[resource-drop]input-stream': trampoline8,
        '[resource-drop]output-stream': trampoline9,
      },
      'wasi:random/insecure-seed@0.2.9': {
        'insecure-seed': exports0['7'],
      },
    }));
    memory0 = exports1.memory;
    realloc0 = exports1.cabi_realloc;
    
    try {
      realloc0Async = WebAssembly.promising(exports1.cabi_realloc);
    } catch(err) {
      realloc0Async = exports1.cabi_realloc;
    }
    
    ({ exports: exports2 } = yield instantiateCore(yield module2, {
      '': {
        $imports: exports0.$imports,
        '0': trampoline19,
        '1': trampoline20,
        '10': trampoline28,
        '11': trampoline29,
        '12': trampoline30,
        '13': trampoline31,
        '14': trampoline32,
        '15': trampoline33,
        '2': trampoline21,
        '3': trampoline19,
        '4': trampoline22,
        '5': trampoline23,
        '6': trampoline24,
        '7': trampoline25,
        '8': trampoline26,
        '9': trampoline27,
      },
    }));
    
    callback_0 = WebAssembly.promising(exports1['[callback][async-lift]semio:framework/checkpoint@1.0.0#checkpoint']);
    callback_0.fnName = "exports1['[callback][async-lift]semio:framework/checkpoint@1.0.0#checkpoint']";
    
    reactor100Poll = WebAssembly.promising(exports1['[async-lift]semio:framework/reactor@1.0.0#poll']);
    jobs100StartJob = WebAssembly.promising(exports1['[async-lift]semio:framework/jobs@1.0.0#start-job']);
    jobs100StepJob = WebAssembly.promising(exports1['[async-lift]semio:framework/jobs@1.0.0#step-job']);
    jobs100CancelJob = WebAssembly.promising(exports1['[async-lift]semio:framework/jobs@1.0.0#cancel-job']);
    checkpoint100Checkpoint = WebAssembly.promising(exports1['[async-lift]semio:framework/checkpoint@1.0.0#checkpoint']);
    checkpoint100Restore = WebAssembly.promising(exports1['[async-lift]semio:framework/checkpoint@1.0.0#restore']);
    describe100Describe = WebAssembly.promising(exports1['[async-lift]semio:framework/describe@1.0.0#describe']);
  })();
  let promise, resolve, reject;
  function runNext (value) {
    try {
      let done;
      do {
        ({ value, done } = gen.next(value));
      } while (!(value instanceof Promise) && !done);
      if (done) {
        if (resolve) resolve(value);
        else return value;
      }
      if (!promise) promise = new Promise((_resolve, _reject) => (resolve = _resolve, reject = _reject));
      value.then(runNext, reject);
    }
    catch (e) {
      if (reject) reject(e);
      else throw e;
    }
  }
  const maybeSyncReturn = runNext(null);
  return promise || maybeSyncReturn;
})();

await $init;
const checkpoint100 = {
  checkpoint: checkpoint,
  restore: restore,
  
};
const describe100 = {
  describe: describe,
  
};
const jobs100 = {
  cancelJob: cancelJob,
  startJob: startJob,
  stepJob: stepJob,
  
};
const reactor100 = {
  poll: poll$1,
  
};

export { checkpoint100 as checkpoint, describe100 as describe, jobs100 as jobs, reactor100 as reactor, checkpoint100 as 'semio:framework/checkpoint@1.0.0', describe100 as 'semio:framework/describe@1.0.0', jobs100 as 'semio:framework/jobs@1.0.0', reactor100 as 'semio:framework/reactor@1.0.0',  }
export const _util = {
  
}

