import os

file_path = '/workspaces/semio/@semio-repo/cli/main.go'
with open(file_path, 'r') as f:
    lines = f.readlines()

def fix_content(content):
    # This is a bit risky but let's try to find ctx.CreateViolation calls
    # that only have 5 arguments after the change.
    
    # We look for ctx.CreateViolation(
    # Then we find the balanced closing parenthesis.
    
    new_content = ""
    i = 0
    while i < len(content):
        if content.startswith("ctx.CreateViolation(", i):
            # Find the balanced closing paren
            start = i
            i += len("ctx.CreateViolation(")
            depth = 1
            while i < len(content) and depth > 0:
                if content[i] == '(':
                    depth += 1
                elif content[i] == ')':
                    depth -= 1
                i += 1
            
            call_content = content[start:i]
            # Now we have something like ctx.CreateViolation(arg1, arg2, arg3, arg4, arg5)
            # We want to check if it has 5 arguments.
            # We can't just split by comma because of nested parens.
            
            # Count commas at the top level of this call
            commas_indices = []
            depth = 0
            # skip 'ctx.CreateViolation('
            for k in range(len("ctx.CreateViolation("), len(call_content) - 1):
                char = call_content[k]
                if char == '(':
                    depth += 1
                elif char == ')':
                    depth -= 1
                elif char == ',' and depth == 0:
                    commas_indices.append(k)
            
            if len(commas_indices) == 4:
                # It has 5 arguments. We need to insert 0 before the last one.
                last_comma = commas_indices[-1]
                new_call = call_content[:last_comma] + ", 0" + call_content[last_comma:]
                new_content += new_call
            else:
                # Already fixed or something else
                new_content += call_content
        else:
            new_content += content[i]
            i += 1
    return new_content

with open(file_path, 'r') as f:
    full_content = f.read()

fixed_content = fix_content(full_content)

with open(file_path, 'w') as f:
    f.write(fixed_content)
